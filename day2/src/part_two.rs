use crate::cube_game::CubeGame;

pub async fn solve(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(CubeGame::from_str)
        .map(|game| game.minimum_cubes_required())
        .map(|sets| {
            sets
                .into_iter()
                .filter(|(_, n)| n.is_some())
                .map(|(_, n)| n.unwrap())
                .reduce(|acc, x| acc * x)
                .unwrap_or(0)
        })
        .sum()
}
