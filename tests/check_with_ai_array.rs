use champing_glass_solution::{GameState, Solution};
use chomping_glass::AI;

#[test]
fn test_ai_array() {
    let solution = Solution::new();
    AI.iter().for_each(|value| {
        let index = value >> 16;
        let next_move = value & 0xFFFF;
        let game_state = GameState::from_index(index as usize);

        if solution.winning_strategy(&game_state).is_some() {
            // The AI array fills initial and ending position with 0xFFFF
            if game_state != GameState([0, 0, 0, 0, 0]) && game_state != GameState([8, 8, 8, 8, 8])
            {
                assert_ne!(next_move, 0xFFFF);
            }
        } else {
            assert_eq!(next_move, 0xFFFF);
        }
    });
}
