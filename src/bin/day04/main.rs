use std::collections::{HashMap, HashSet};
use std::fs;

type Point = (usize, usize);

fn main() {
  let filename = "src/bin/day04/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let mut grid: HashMap<Point, char> = parse_grid(&input);

  // iterate over grid
  let mut to_be_removed = get_points_accessible_to_forklift(&grid);
  check_part_1_answer(filename, to_be_removed.len());

  let mut num_removed = 0;
  while !to_be_removed.is_empty() {
    grid.retain(|p, _| !to_be_removed.contains(p));
    num_removed += to_be_removed.len();
    to_be_removed = get_points_accessible_to_forklift(&grid);
  }
  check_part_2_answer(filename, num_removed);
}

fn get_points_accessible_to_forklift(grid: &HashMap<Point, char>) -> HashSet<Point> {
  grid
    .iter()
    .filter(|((x, y), c)| {
      if **c != '@' {
        return false;
      }
      let num_neighboring_toilet_papers = neighbors(*x, *y)
        .filter(|p| grid.get(p).is_some_and(|&c| c == '@'))
        .count();
      num_neighboring_toilet_papers < 4
    })
    .map(|(p, _)| *p)
    .collect()
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = Point> {
  [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
  ]
  .into_iter()
  .filter_map(move |(dx, dy)| Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?)))
}

fn check_part_1_answer(filename: &str, answer: usize) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day04/input-fake-ignore.txt" => assert_eq!(answer, 13),
    "src/bin/day04/input-real-ignore.txt" => assert_eq!(answer, 1437),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: usize) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day04/input-fake-ignore.txt" => assert_eq!(answer, 43),
    "src/bin/day04/input-real-ignore.txt" => assert_eq!(answer, 8765),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_grid(input: &str) -> HashMap<Point, char> {
  input
    .lines()
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
    .collect()
}
