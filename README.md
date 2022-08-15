# CHESS DANGER ANALYZER

## WHAT DOES IT DO???
In the beggining of src/main.rs you will see a board thingy, this is the input
to the program. If you `cargo run` in the root directory it will output the moves your opponent can make to get 1 move away from eating any of your pieces (it will not output
moves that immediately eat a piece of yours).

The output is formated with the [bunt](https://crates.io/crates/bunt/0.2.6) crate, making it somewhat easy to understand what is going on.

### The Input
The input is a vector of arrays (8x8), with characters that can be either a space or a letter. Each letter represent a piece. If the letter is UPPERCASE, it's your opponents piece, and if it's lowercase, it's your piece. Here is what each letter represent:

- P: pawns
- N: knights
- T: Towers
- B: Bishops
- Q: Queens
- K: Kings

### The Output
For each threatening move, the console will display a message that looks like this:

if the opponent moves his knight to f3,
he will be able to eat your queen on e4

_BOARD GOES HERE_

First we see a description of the threat, and then we see the board (here on github it does not work, but it's cool on the console i swear). The # will show where the enemy piece initially was, the blue-ish letter will be the piece that he moved, and the pink letter will be your piece that is about to be eaten.

## why did i do this?
I was playing a online chess game with a friend that is much better than me, so i thought that maybe writing a "hack" i could get closer to his skill level while also learning a bit more about rust :)

## Is it finished?
No
