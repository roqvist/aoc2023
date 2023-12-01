use tokio::fs;

mod part_one;
mod part_two;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = fs::read_to_string("puzzleinput.txt").await.expect("Error reading puzzle input");

    let output = part_one::solve(&puzzle_input).await;
    println!("Part one: {output}");

    let output = part_two::solve(&puzzle_input).await;
    println!("Part two: {output}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use tokio::fs;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn part_one_test() {
        let puzzle_input = fs::read_to_string("puzzleinputtest.txt").await.expect("Error reading puzzle input");
        let output = part_one::solve(&puzzle_input).await;
        assert_eq!(output, 142);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn part_two_test() {
        let puzzle_input = fs::read_to_string("puzzleinputtest2.txt").await.expect("Error reading puzzle input");
        let output = part_two::solve(&puzzle_input).await;
        assert_eq!(output, 281);

        let puzzle_input = fs::read_to_string("puzzleinputtest3.txt").await.expect("Error reading puzzle input");
        let output = part_two::solve(&puzzle_input).await;
        assert_eq!(output, 303);
    }
}
