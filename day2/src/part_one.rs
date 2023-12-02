use std::collections::HashMap;

use crate::cube_game::{CubeGame, CubeColor};

pub async fn solve(puzzle_input: &str) -> i32 {
    let bag_contents = HashMap::from([
        (CubeColor::Red, 12),
        (CubeColor::Green, 13),
        (CubeColor::Blue, 14)
    ]);

    puzzle_input
        .lines()
        .map(CubeGame::from_str)
        .filter(|game| game.can_be_played_with_bag(&bag_contents))
        .map(|playable| playable.id)
        .sum()
}
