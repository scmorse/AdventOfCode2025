mod max_by_first_key;

use max_by_first_key::MaxByFirstKey;
use std::fs;

fn main() {
  let filename = "src/bin/day03/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let grid: Vec<Vec<u32>> = parse_input_lines(input);

  let part_1_answer: u32 = grid
    .iter()
    .map(|row| {
      let first = row.max_by_first_key(0, row.len() - 1);
      let last = row.max_by_first_key(first + 1, row.len());
      row[first] * 10 + row[last]
    })
    .sum();
  check_part_1_answer(filename, part_1_answer);

  let part_2_answer: u64 = grid
    .iter()
    .map(|row| {
      let mut indices = Vec::with_capacity(12);
      let mut start = 0;
      for remaining in (0..12).rev() {
        let idx = row.max_by_first_key(start, row.len() - remaining);
        indices.push(idx);
        start = idx + 1;
      }
      let result: String = indices
        .iter()
        .map(|&i| char::from_digit(row[i], 10).unwrap())
        .collect();
      result.parse::<u64>().unwrap()
    })
    .sum();
  check_part_2_answer(filename, part_2_answer);
}

fn check_part_1_answer(filename: &str, answer: u32) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day03/input-fake-ignore.txt" => assert_eq!(answer, 357),
    "src/bin/day03/input-real-ignore.txt" => assert_eq!(answer, 17321),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: u64) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day03/input-fake-ignore.txt" => assert_eq!(answer, 3121910778619),
    "src/bin/day03/input-real-ignore.txt" => assert_eq!(answer, 171989894144198),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input_lines(input: String) -> Vec<Vec<u32>> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect()
    })
    .collect()
}
