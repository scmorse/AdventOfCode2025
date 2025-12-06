mod column;

use column::{Column, Op};
use std::fs;

fn main() {
  let filename = "src/bin/day06/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");

  let (numbers, ops) = parse_input_v1(&input);
  let part_1_answer: u64 = ops
    .iter()
    .enumerate()
    .map(|(i, op)| {
      numbers
        .iter()
        .map(|row| row[i])
        .reduce(|a, b| op.apply(a, b))
        .unwrap()
    })
    .sum();
  check_part_1_answer(filename, part_1_answer);

  let columns = parse_input_v2(&input);
  let groups: Vec<&[Column]> = columns
    .split(|c| matches!(c, Column::Reset))
    .filter(|group| !group.is_empty())
    .collect();
  let part_2_answer: u64 = groups
    .iter()
    .map(|g| {
      let Some(Column::ValueAndOp(_, op)) = g.first() else {
        unreachable!()
      };
      g.iter()
        .map(|column| column.value().expect("already filtered out reset columns"))
        .reduce(|a, b| op.apply(a, b))
        .unwrap()
    })
    .sum();
  check_part_2_answer(filename, part_2_answer);
}

fn check_part_1_answer(filename: &str, answer: u64) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day06/input-fake-ignore.txt" => assert_eq!(answer, 4277556),
    "src/bin/day06/input-real-ignore.txt" => assert_eq!(answer, 7229350537438),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: u64) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day06/input-fake-ignore.txt" => assert_eq!(answer, 3263827),
    "src/bin/day06/input-real-ignore.txt" => assert_eq!(answer, 11479269003550),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input_v2(input: &str) -> Vec<Column> {
  let lines: Vec<&str> = input.lines().collect();
  let (number_lines, ops_line) = lines.split_at(lines.len() - 1);
  let ops_chars: Vec<char> = ops_line[0].chars().collect();

  // Find max line length to know how many columns to iterate
  let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

  let mut columns = Vec::with_capacity(max_len);

  for col in 0..max_len {
    // Collect non-whitespace chars from all number lines at this column
    let digit_chars: String = number_lines
      .iter()
      .filter_map(|line| line.chars().nth(col))
      .filter(|c| !c.is_whitespace())
      .collect();

    // Get the operator char at this column (if it exists)
    let op_char = ops_chars.get(col).copied();

    let column = match (digit_chars.is_empty(), op_char.and_then(Op::from_char)) {
      (true, _) => Column::Reset,
      (false, Some(op)) => {
        let value: u64 = digit_chars.parse().expect("Invalid number");
        Column::ValueAndOp(value, op)
      }
      (false, None) => {
        let value: u64 = digit_chars.parse().expect("Invalid number");
        Column::Value(value)
      }
    };

    columns.push(column);
  }

  columns
}

fn parse_input_v1(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
  let lines: Vec<&str> = input.lines().collect();
  let (number_lines, ops_line) = lines.split_at(lines.len() - 1);

  let numbers: Vec<Vec<u64>> = number_lines
    .iter()
    .map(|line| {
      line
        .split_whitespace()
        .map(|t| t.parse().expect("Invalid number"))
        .collect()
    })
    .collect();

  let ops: Vec<Op> = ops_line[0]
    .split_whitespace()
    .map(|t| {
      let c = t.chars().next().expect("Empty token");
      Op::from_char(c).unwrap_or_else(|| panic!("Unknown op: {t}"))
    })
    .collect();

  assert!(
    numbers.iter().all(|row| row.len() == ops.len()),
    "All number rows must have the same length as ops"
  );

  (numbers, ops)
}
