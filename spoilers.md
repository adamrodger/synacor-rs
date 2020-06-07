# Spoilers

- The first code is in the documentation itself
- The second code is printed at the start of the program
- The third code is printed after the self-test completes successfully
- The fourth code is on the tablet when you enter the game: `take tablet` then `use tablet`
- The fifth code is scrawled on the wall in the maze where you find the can:

```text
doorway
north
north
bridge
continue
down
east
take empty lantern
west
west
passage
ladder
west
south
north
```

- Get to the door and collect all the coins required

```text
take can
use can
west
ladder
darkness
use lantern
continue
west
west
west
west
north
take red coin
north
west
take blue coin
up
take shiny coin
down
east
east
take concave coin
down
take corroded coin
up
west
```

- Unlock the door by trying the different permutations of the coins (P(5,5) = 120) (`cargo run --bin solve_coins`).
  The right one is:

```text
use blue coin
use red coin
use shiny coin
use concave coin
use corroded coin
```

- The sixth code is printed after you go through the unlocked door and use the teleporter:

```text
north
take teleporter
use teleporter
```

- Take the book and read it to get the next challenge hint where we need to work out what to set the '8th register' to
  and also how to bypass the check:

```text
take strange book
look strange book
```
