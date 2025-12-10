use advent_of_code2025::pairs::Pairs;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

type Point = (usize, usize, usize);

fn main() {
  let filename = "src/bin/day08/input-real-ignore.txt";
  let num_closest_boxes = match filename {
    "src/bin/day08/input-fake-ignore.txt" => 10usize,
    "src/bin/day08/input-real-ignore.txt" => 1000usize,
    _ => panic!("Unexpected filename: {filename}"),
  };
  let input = fs::read_to_string(filename).expect("Failed to read input file");

  let points: Vec<Point> = parse_input(&input);
  let mut pairs: Vec<(Point, Point)> = points.pairs();
  pairs.sort_by_key(|&((ax, ay, az), (bx, by, bz))| {
    (max(ax, bx) - min(ax, bx)).pow(2)
      + (max(ay, by) - min(ay, by)).pow(2)
      + (max(az, bz) - min(az, bz)).pow(2)
  });

  let part_1_answer = part_1(&points, &pairs, num_closest_boxes);
  check_part_1_answer(filename, part_1_answer);

  let part_2_answer = part_2(&points, &pairs);
  check_part_2_answer(filename, part_2_answer);
}

fn part_1(points: &Vec<Point>, pairs: &Vec<(Point, Point)>, num_closest_boxes: usize) -> usize {
  let mut parents: HashMap<Point, Point> = HashMap::new();
  // start off with all points being their own parents
  for point in points.iter() {
    parents.insert(*point, *point);
  }

  pairs[..num_closest_boxes].iter().for_each(|(a, b)| {
    union(&mut parents, *a, *b);
  });
  let mut groups: Vec<Vec<Point>> = points
    .iter()
    .into_group_map_by(|&point| find_current_parent(&parents, *point))
    .into_values()
    .map(|group| group.into_iter().copied().collect())
    .collect();
  groups.sort_by_key(|group| group.len());
  groups
    .iter()
    .rev()
    .take(3)
    .map(|group| group.len())
    .reduce(|a, b| a * b)
    .expect("Failed to calculate part 1 answer")
}

fn part_2(points: &Vec<Point>, pairs: &Vec<(Point, Point)>) -> usize {
  let mut parents: HashMap<Point, Point> = HashMap::new();
  // start off with all points being their own parents
  for point in points.iter() {
    parents.insert(*point, *point);
  }

  for (i, (a, b)) in pairs.iter().enumerate() {
    union(&mut parents, *a, *b);
    if i <= points.len() {
      continue;
    }
    let num_groups = points
      .iter()
      .into_group_map_by(|&point| find_current_parent(&parents, *point))
      .into_values()
      .count();
    if num_groups == 1 {
      return a.0 * b.0;
    }
  }
  unreachable!("Failed to find part 2 answer")
}

fn find_current_parent(parents: &HashMap<Point, Point>, point: Point) -> Point {
  let above = parents[&point];
  if above == point {
    return point;
  }
  find_current_parent(parents, above)
}

fn union(parents: &mut HashMap<Point, Point>, a: Point, b: Point) {
  let a_parent = find_current_parent(parents, a);
  let b_parent = find_current_parent(parents, b);
  parents.insert(a_parent, b_parent);
}

fn check_part_1_answer(filename: &str, answer: usize) {
  println!("Part 1 answer: {answer}");
  match filename {
    "src/bin/day08/input-fake-ignore.txt" => assert_eq!(answer, 40),
    "src/bin/day08/input-real-ignore.txt" => assert_eq!(answer, 123420),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn check_part_2_answer(filename: &str, answer: usize) {
  println!("Part 2 answer: {answer}");
  match filename {
    "src/bin/day08/input-fake-ignore.txt" => assert_eq!(answer, 25272),
    "src/bin/day08/input-real-ignore.txt" => assert_eq!(answer, 673096646),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_input(input: &str) -> Vec<Point> {
  input
    .split('\n')
    .map(|line| {
      let [x, y, z] = line.split(",").collect::<Vec<_>>()[..] else {
        panic!("Invalid line: {:?}", line)
      };
      (
        x.parse::<usize>().expect("Failed to parse x coordinate"),
        y.parse::<usize>().expect("Failed to parse y coordinate"),
        z.parse::<usize>().expect("Failed to parse z coordinate"),
      )
    })
    .collect::<Vec<Point>>()
}
