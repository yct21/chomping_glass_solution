const HEIGHT: usize = 5;
const WIDTH: usize = 8;
const MARKS_SIZE: usize = 1 << 16; // 65536

#[derive(Copy, Clone, Debug, PartialEq)]
enum PointType {
    Unknown,
    NextWin(u8, u8),
    PreviousWin,
}

/// Contains amount of eaten candies in each row
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GameState(pub [u8; HEIGHT]);

impl GameState {
    pub fn to_index(&self) -> usize {
        let mut index = 0;
        let mut t = 8;
        self.0.iter().for_each(|&eaten| {
            if t > eaten {
                index <<= t - eaten;
                t = eaten;
            }
            index <<= 1;
            index |= 0x01;
        });
        index <<= t;

        index
    }

    pub fn from_index(mut index: usize) -> Self {
        let mut state = [0u8; HEIGHT];

        state[HEIGHT - 1] = index.trailing_zeros() as u8;
        index >>= state[HEIGHT - 1] + 1;

        let mut current_zeros = 0;
        let mut current_height = HEIGHT - 1;

        while index != 0 {
            if index & 0x01 == 1 {
                current_height -= 1;
                state[current_height] = state[current_height + 1] + current_zeros;
                current_zeros = 0;
                index >>= 1;
            } else {
                current_zeros += 1;
                index >>= 1;
            }
        }

        Self(state)
    }
}

pub struct Solution {
    /// n/p marks of the game
    /// A true value marks a P position
    marks: [PointType; MARKS_SIZE],
}

impl Solution {
    pub fn new() -> Self {
        let mut marks = [PointType::Unknown; MARKS_SIZE];

        marks[0b1111100000000] = PointType::NextWin(0xFF, 0xFF); // All candies eaten
        marks[0b1111010000000] = PointType::PreviousWin; // All candies eaten except bottom-right

        // memoize search
        fn solve(current_game_index: usize, marks: &mut [PointType]) {
            let current_state = GameState::from_index(current_game_index);
            let mut previous_win_point_found = false;

            for i in 0..HEIGHT as u8 {
                for j in (current_state.0[i as usize] + 1)..=WIDTH as u8 {
                    let mut next_state = current_state.clone();
                    for k in 0..=i {
                        next_state.0[k as usize] = next_state.0[k as usize].max(j);
                    }
                    let next_state_index = next_state.to_index();

                    if marks[next_state_index as usize] == PointType::Unknown {
                        solve(next_state_index, marks);
                        assert!(marks[next_state_index as usize] != PointType::Unknown);
                    }

                    if marks[next_state.to_index() as usize] == PointType::PreviousWin {
                        marks[current_state.to_index() as usize] = PointType::NextWin(i, j);
                        previous_win_point_found = true;
                        // break here leads to unreachable holes in the `marks` array
                    }
                }
            }

            if !previous_win_point_found {
                marks[current_state.to_index() as usize] = PointType::PreviousWin;
            }
        }

        solve(0b11111, &mut marks); // Initial state

        Self { marks }
    }

    pub fn winning_strategy(&self, game: &GameState) -> Option<(usize, usize)> {
        let game_index = game.to_index();
        match self.marks[game_index] {
            PointType::Unknown => unreachable!(),
            PointType::NextWin(row, column) => Some((row as usize, column as usize)),
            PointType::PreviousWin => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state() {
        let state = GameState([8, 8, 8, 8, 8]);
        assert_eq!(state.to_index(), 0b1111100000000);
        assert_eq!(GameState::from_index(0b1111100000000), state);

        let state = GameState([8, 8, 8, 8, 7]);
        assert_eq!(state.to_index(), 0b1111010000000);
        assert_eq!(GameState::from_index(0b1111010000000), state);

        let state = GameState([8, 8, 8, 8, 0]);
        assert_eq!(state.to_index(), 0b1111000000001);
        assert_eq!(GameState::from_index(0b1111000000001), state);

        let state = GameState([8, 8, 8, 0, 0]);
        assert_eq!(state.to_index(), 0b1110000000011);
        assert_eq!(GameState::from_index(0b1110000000011), state);

        let state = GameState([0, 0, 0, 0, 0]);
        assert_eq!(state.to_index(), 0b11111);
        assert_eq!(GameState::from_index(0b11111), state);

        let state = GameState([4, 3, 2, 1, 0]);
        assert_eq!(state.to_index(), 0b101010101);
        assert_eq!(GameState::from_index(0b101010101), state);

        let state = GameState([8, 6, 4, 2, 0]);
        assert_eq!(state.to_index(), 0b1001001001001);
        assert_eq!(GameState::from_index(0b1001001001001), state);
    }
}
