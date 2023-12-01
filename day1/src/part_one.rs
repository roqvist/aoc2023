pub async fn solve(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(|l| {
            let first_numeric_char = l.chars().find(|c| c.is_numeric()).expect("No numeric char in line");
            let last_numeric_char = l.chars().rev().find(|c| c.is_numeric()).expect("No numeric char in line");

            format!("{}{}", first_numeric_char, last_numeric_char).parse::<i32>().unwrap()
        })
        .sum()
}
