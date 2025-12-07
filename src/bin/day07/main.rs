use std::collections::HashMap;
use std::fs;
use tap::Tap;

type Point = (usize, usize);
type Grid = HashMap<Point, char>;

fn main() {
  let filename = "src/bin/day07/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let (grid, start) = parse_input(&input);

  let mut memoized: HashMap<Point, usize> = HashMap::new();
  let part_2_answer = count_paths(start, &grid, &mut memoized);
  let part_1_answer = memoized.len();
  check_part_1_answer(filename, part_1_answer);
  check_part_2_answer(filename, part_2_answer);
}

fn count_paths((x, y): Point, grid: &Grid, memoized: &mut HashMap<Point, usize>) -> usize {
  match grid.get(&(x, y)) {
    Some('.') | Some('S') => count_paths((x, y + 1), grid, memoized),
    Some('^') => match memoized.get(&(x, y)) {
      Some(&cached) => cached,
      None => (count_paths((x - 1, y), grid, memoized) + count_paths((x + 1, y), grid, memoized))
        .tap(|count| _ = memoized.insert((x, y), *count)),
    },
    Some(_) => unreachable!("Invalid char at ({x}, {y}): {:?}", grid.get(&(x, y))),
    None => 1,
  }
}

fn check_part_1_answer(filename: &str, answer: usize) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day07/input-fake-ignore.txt" => assert_eq!(answer, 21),
    "src/bin/day07/input-real-ignore.txt" => assert_eq!(answer, 1681),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: usize) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day07/input-fake-ignore.txt" => assert_eq!(answer, 40),
    "src/bin/day07/input-real-ignore.txt" => assert_eq!(answer, 422102272495018),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input(input: &str) -> (Grid, Point) {
  let lines: Vec<&str> = input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect();
  let mut grid: Grid = HashMap::new();
  let mut start: Option<Point> = None;

  for (y, line) in lines.iter().enumerate() {
    for (x, &byte) in line.as_bytes().iter().enumerate() {
      grid.insert((x, y), byte as char);
      if byte == b'S' {
        start = Some((x, y));
      }
    }
  }
  (grid, start.expect("No start position found"))
}
