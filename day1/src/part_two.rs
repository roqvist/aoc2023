use std::collections::HashMap;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub async fn solve(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(|l| {
            let first_numeric = l
                .chars()
                .enumerate()
                .find(|(_, c)| c.is_numeric())
                .expect("No numeric char in forward line");
            let last_numeric = l
                .chars()
                .rev()
                .enumerate()
                .find_map(|(i, c)| if c.is_numeric() { Some((l.len() - 1 - i, c)) } else { None })
                .expect("No numeric char in backward line");

            let mut number_words: HashMap<usize, char> = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    if let Some(j) = l.find(n) {
                        Some((j, format!("{}", i + 1).parse::<char>().unwrap()))
                    } else {
                        None
                    }
                })
                .collect();

            let reverse_number_words: HashMap<usize, char> = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    let reversed_l = l.chars().rev().collect::<String>();
                    let reversed_n = n.chars().rev().collect::<String>();
                    if let Some(j) = reversed_l.find(&reversed_n) {
                        Some((l.len() - 1 - j, format!("{}", i + 1).parse::<char>().unwrap()))
                    } else {
                        None
                    }
                })
                .collect();

            number_words.extend(reverse_number_words);
            let lowest_word = number_words
                .clone()
                .into_iter()
                .min_by_key(|n| n.0.to_owned())
                .unwrap_or(first_numeric.clone());

            let highest_word = number_words
                .clone()
                .into_iter()
                .max_by_key(|n| n.0.to_owned())
                .unwrap_or(last_numeric.clone());

            let lowest = [first_numeric, last_numeric, lowest_word, highest_word]
                .into_iter()
                .min_by_key(|n| n.to_owned())
                .unwrap();

            let highest = [first_numeric, last_numeric, lowest_word, highest_word]
                .into_iter()
                .max_by_key(|n| n.to_owned())
                .unwrap();

            format!("{}{}", lowest.1, highest.1).parse::<i32>().unwrap()
        })
        .sum()
}
