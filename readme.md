# Chomping Glass Solution

Solution to the Chomping Glass problem in [this twitter thread](https://twitter.com/jarxiao/status/1700179078014185789) and [repository](https://github.com/jarry-xiao/chomping-glass).

## Answers to additional questions

### How does the AI work?

The AI contains an array with precalculated answers for any game state. 

Each element in the `AI` array is a combination of

- game state (first 16 bits): 
  - `1`s are seperators 
  - last few `0`s are number of eaten candies in last row
  - other `0`s are the substraction difference between the number eaten candies of each adjacent rows
- winning move (last 16 bits): 
  - 0xFFFF if there's none
    - excepts for the initial game state since the AI always comes as the second player
    - and game state with no candy left since the game is already over

When invoked, it queries the array using a binary search to find a element with first 16 bits the same as current game state. It then returns the winning move if there is any by checking the last 16 bits of the value. Otherwise it returns a random move.

### WTF is this magic 1287 number?

It is the amount of valid states.

```rust
fn valid_state_count(x: u64, y: u64) -> u64 {
    if y == 1 {
        return x + 1;
    } else {
        let mut sum = 0u64;
        for i in 0..=x {
            sum += valid_state_count(i, y - 1);
        }

        return sum;
    }
}

fn main() {
    print!("{}", valid_state_count(5, 8)); // => 1287
}
```

### How did I generate this code?

The `AI` array can be generated with assigning N-positions and P-positions to each game state.

### Can you write a program that beats the bot every time?

[Solution](./src/lib.rs)
