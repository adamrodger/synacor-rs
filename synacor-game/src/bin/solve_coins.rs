use itertools::Itertools;

/// Solves _ + _ * _^2 + _^3 - _ = 399
fn main() {
    let coins = [
        ("red", 2usize),
        ("corroded", 3),
        ("shiny", 5),
        ("concave", 7),
        ("blue", 9),
    ];

    let answer = coins.iter()
        .permutations(5)
        .find(|p| p[0].1 + p[1].1 * p[2].1.pow(2) + p[3].1.pow(3) - p[4].1 == 399)
        .unwrap();

    for coin in answer {
        println!("use {} coin", coin.0);
    }
}
