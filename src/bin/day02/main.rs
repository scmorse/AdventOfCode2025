use num_integer::div_rem;
use std::fs;

fn main() {
  let filename = "src/bin/day02/input-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let pairs: Vec<(&str, &str)> = parse_input_pairs(&input);

  let part_1_answer: u64 = pairs
    .iter()
    .flat_map(|(left, right)| left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap())
    .filter(|id| is_invalid_id_for_part_1(&id.to_string()))
    .sum();
  check_part_1_answer(filename, part_1_answer);

  let part_2_answer: u64 = pairs
    .iter()
    .flat_map(|(left, right)| left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap())
    .filter(|id| is_invalid_id_for_part_2(&id.to_string()))
    .sum();
  check_part_2_answer(filename, part_2_answer);
}

fn is_invalid_id_for_part_1(s: &str) -> bool {
  if s.len() % 2 != 0 {
    return false;
  }
  is_invalid_id_for_pattern_size(s, s.len() / 2)
}

fn is_invalid_id_for_part_2(s: &str) -> bool {
  for pattern_size in 1..s.len() {
    if is_invalid_id_for_pattern_size(s, pattern_size) {
      return true;
    }
  }
  false
}

// 123123123
// q=3, r=0
// s[0..3] == s[3..6]
// s[3..6] == s[6..9]
fn is_invalid_id_for_pattern_size(s: &str, pattern_size: usize) -> bool {
  let (quotient, remainder) = div_rem(s.len(), pattern_size);
  if remainder != 0 {
    return false;
  }
  if quotient < 2 {
    return false;
  }
  let first_chunk = &s[0..pattern_size];
  for i in 1..quotient {
    let chunk = &s[(i * pattern_size)..(i + 1) * pattern_size];
    if chunk != first_chunk {
      return false;
    }
  }
  true
}

fn check_part_1_answer(filename: &str, answer: u64) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day02/input-fake-ignore.txt" => assert_eq!(answer, 1227775554),
    "src/bin/day02/input-ignore.txt" => assert_eq!(answer, 30599400849),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: u64) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day02/input-fake-ignore.txt" => assert_eq!(answer, 4174379265),
    "src/bin/day02/input-ignore.txt" => assert_eq!(answer, 46270373595),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input_pairs(input: &str) -> Vec<(&str, &str)> {
  input
    .lines()
    .flat_map(|line| line.split(','))
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(parse_pair)
    .collect()
}

fn parse_pair(s: &str) -> (&str, &str) {
  let Some((left, right)) = s.split_once('-') else {
    panic!("Expected 'str1-str2' format, got: {s}");
  };
  (left, right)
}
