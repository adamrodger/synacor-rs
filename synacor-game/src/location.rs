extern crate nom;

use std::{error, fmt, str::FromStr};

/// an item
pub type Item = String;

/// location ID
pub type LocationId = String;

#[derive(Debug, PartialEq)]
/// a location within the maze
pub struct Location {
    id: LocationId,
    description: String,
    items: Vec<Item>,
    exits: Vec<LocationId>,
}

impl Location {
    /// Location ID
    pub fn id(&self) -> &LocationId {
        &self.id
    }
}

impl FromStr for Location {
    type Err = LocationError;

    fn from_str(s: &str) -> Result<Self, LocationError> {
        let (_, location) = parser::parse(s)?;
        Ok(location)
    }
}

#[derive(Debug)]
/// Location errors
pub enum LocationError {
    ParseError,
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocationError")
    }
}

impl error::Error for LocationError {}

impl From<nom::Err<(&str, nom::error::ErrorKind)>> for LocationError {
    fn from(_: nom::Err<(&str, nom::error::ErrorKind)>) -> Self {
        LocationError::ParseError
    }
}

/// Location parser
pub(self) mod parser {
    use super::{Item, Location, LocationId};
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{digit1, newline},
        combinator::{rest, opt},
        multi::many1,
        sequence::{delimited, tuple},
    };

    /// Parse a string into a location
    pub fn parse(s: &str) -> nom::IResult<&str, Location> {
        // skip any lead-in, like the VM self test, until the first location starts
        let (s, _) = opt(take_until("=="))(s)?;

        let (s, (id, _, description, _, _, items, _, exits, _)) = tuple((
            parse_id,
            newline,
            parse_description,
            newline,
            newline,
            parse_items,
            opt(newline),
            parse_exits,
            rest,
        ))(s)?;

        Ok((
            s,
            Location {
                id,
                description,
                items,
                exits,
            },
        ))
    }

    /// Parse the ID portion of a location - e.g. "== Big Room =="
    fn parse_id(s: &str) -> nom::IResult<&str, LocationId> {
        let (s, id) = delimited(tag("== "), take_until(" ="), tag(" =="))(s)?;
        Ok((s, id.into()))
    }

    /// Parse the description of the location
    fn parse_description(s: &str) -> nom::IResult<&str, String> {
        let (s, description) = take_until("\n")(s)?;
        Ok((s, description.into()))
    }

    /// Parse the list of items at the location
    fn parse_items(s: &str) -> nom::IResult<&str, Vec<Item>> {
        let (s, header) = opt(tag("Things of interest here:\n"))(s)?;

        // check if room has no items
        if header.is_none() {
            return Ok((s, Vec::new()))
        }

        let item_parser = delimited(tag("- "), take_until("\n"), newline);
        let (s, items) = many1(item_parser)(s)?;

        Ok((s, items.into_iter().map(|s| s.into()).collect()))
    }

    /// Parse the list of exits from the location
    fn parse_exits(s: &str) -> nom::IResult<&str, Vec<LocationId>> {
        let (s, _) = tuple((tag("There are "), digit1, tag(" exits:"), newline))(s)?;

        let exit_parser = delimited(tag("- "), take_until("\n"), newline);
        let (s, items) = many1(exit_parser)(s)?;

        Ok((s, items.into_iter().map(|s| s.into()).collect()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_id_single_word() {
            let expected: nom::IResult<&str, LocationId> = Ok(("", "Single".into()));

            let actual = parse_id("== Single ==");

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_id_multi_word() {
            let expected: nom::IResult<&str, LocationId> = Ok(("", "Multi Word".into()));

            let actual = parse_id("== Multi Word ==");

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_id_with_suffix() {
            let expected = Ok((" Suffix", "With".into()));

            let actual = parse_id("== With == Suffix");

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_id_with_newline() {
            let expected = Ok(("\n", "Single".into()));

            let actual = parse_id("== Single ==\n");

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_description() {
            let expected: nom::IResult<&str, String> = Ok(("\n", "A full description".into()));

            let actual = parse_description("A full description\n");

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_items_single() {
            let input = "Things of interest here:\n\
                 - book\n";
            let expected: nom::IResult<&str, Vec<Item>> = Ok(("", vec!["book".into()]));

            let actual = parse_items(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_items_multi() {
            let input = "Things of interest here:\n\
                 - large bag\n\
                 - dagger\n\
                 - piece of fruit\n";
            let expected: nom::IResult<&str, Vec<Item>> = Ok((
                "",
                vec!["large bag".into(), "dagger".into(), "piece of fruit".into()],
            ));

            let actual = parse_items(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_items_none() {
            let input = "\n";
            let expected: nom::IResult<&str, Vec<Item>> = Ok(("\n", Vec::new()));

            let actual = parse_items(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_exits_single() {
            let input = "There is 1 exit:\n\
                 - north\n";
            let expected: nom::IResult<&str, Vec<Item>> = Ok(("", vec!["north".into()]));

            let actual = parse_items(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_exits_multi() {
            let input = "There are 3 exits:\n\
                 - north\n\
                 - south\n\
                 - large door\n";
            let expected: nom::IResult<&str, Vec<Item>> = Ok((
                "",
                vec!["north".into(), "south".into(), "large door".into()],
            ));

            let actual = parse_items(input);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_happy_path() {
            let input = "\
== Foothills ==
You find yourself standing at the base of an enormous mountain.  At its base to the north, there is a massive doorway.  A sign nearby reads \"Keep out!  Definitely no treasure within!\"

Things of interest here:
- tablet

There are 2 exits:
- doorway
- south

What do you do?
";

            let expected = Location {
                id: "Foothills".into(),
                description: "You find yourself standing at the base of an enormous mountain.  At its base to the north, there is a massive doorway.  A sign nearby reads \"Keep out!  Definitely no treasure within!\"".into(),
                items: vec!["tablet".into()],
                exits: vec!["doorway".into(), "south".into()],
            };

            let (remaining, actual) = parse(input).unwrap();

            assert_eq!(remaining, "");
            assert_eq!(actual, expected);
        }
    }
}
