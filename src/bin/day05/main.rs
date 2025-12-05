use std::cmp::max;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
  let filename = "src/bin/day05/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let (ranges, numbers) = parse_input(&input);

  let part_1_answer = numbers
    .iter()
    .filter(|&&n| ranges.iter().any(|r| r.contains(&n)))
    .count();
  check_part_1_answer(filename, part_1_answer);

  let mut curr_ranges = ranges.clone();
  while let Some(next_ranges) = reduce_with_overlap(&curr_ranges) {
    curr_ranges = next_ranges;
  }
  let part_2_answer: usize = curr_ranges.iter().map(|r| r.end() - r.start() + 1).sum();
  check_part_2_answer(filename, part_2_answer);
}

fn reduce_with_overlap(
  curr_ranges: &[RangeInclusive<usize>],
) -> Option<Vec<RangeInclusive<usize>>> {
  for (i1, r1) in curr_ranges.iter().enumerate() {
    for (i2, r2) in curr_ranges[i1 + 1..].iter().enumerate() {
      if let Some(combined) = combine_ranges(r1, r2) {
        let mut next_ranges = curr_ranges.to_vec();
        next_ranges.remove(i1 + 1 + i2);
        next_ranges.remove(i1);
        next_ranges.push(combined);
        return Some(next_ranges);
      }
    }
  }
  None
}

fn combine_ranges(
  a: &RangeInclusive<usize>,
  b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
  if a.start() == b.start() {
    return Some(*a.start()..=max(*a.end(), *b.end()));
  }
  let (lower, higher) = if a.start() < b.start() {
    (a, b)
  } else {
    (b, a)
  };
  if lower.end() > higher.end() {
    return Some(lower.clone());
  }
  // +1 so that adjacent ranges are merged, e.g.-- 1-3 and 4-6 are merged into 1-6
  if *higher.start() <= lower.end() + 1 {
    return Some(*lower.start()..=*higher.end());
  }
  None
}

fn check_part_1_answer(filename: &str, answer: usize) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day05/input-fake-ignore.txt" => assert_eq!(answer, 3),
    "src/bin/day05/input-real-ignore.txt" => assert_eq!(answer, 896),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: usize) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day05/input-fake-ignore.txt" => assert_eq!(answer, 14),
    "src/bin/day05/input-real-ignore.txt" => assert_eq!(answer, 346240317247002),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
  let mut sections = input.split("\n\n");

  let ranges = sections
    .next()
    .expect("Missing ranges section")
    .lines()
    .map(|line| {
      let (start, end) = line.split_once('-').expect("Invalid range format");
      let start: usize = start.parse().expect("Invalid start number");
      let end: usize = end.parse().expect("Invalid end number");
      start..=end
    })
    .collect();

  let numbers = sections
    .next()
    .expect("Missing numbers section")
    .lines()
    .map(|line| line.parse().expect("Invalid number"))
    .collect();

  (ranges, numbers)
}
