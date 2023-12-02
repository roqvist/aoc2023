use crate::cube_game::CubeGame;

pub async fn solve(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(CubeGame::from_str)
        .map(|game| game.minimum_cubes_required().into_values())
        .map(|color_values| color_values.reduce(|acc, x| acc * x).unwrap_or(0))
        .sum()
}
