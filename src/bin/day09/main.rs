mod box_with_area;

use crate::box_with_area::Point;
use advent_of_code2025::pairs::Pairs;
use box_with_area::BoxWithArea;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn main() {
  let filename = "src/bin/day09/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let points: Vec<Point> = parse_input(&input);

  let pairs = points.pairs();
  let points_max_area = pairs
    .iter()
    .max_by_key(|(a, b)| area(a, b))
    .expect("Failed to find max area");
  println!(
    "part 1 answer: {:?}",
    area(&points_max_area.0, &points_max_area.1)
  );

  let perim: HashSet<Point> = perimeter(&points).into_iter().collect::<HashSet<_>>();
  // zip with next element in pairs
  let mut lefts: Vec<Point> = Vec::new();
  let mut rights: Vec<Point> = Vec::new();
  for (p1, p2) in zip_with_next(&points) {
    let (l, r) = adjacent_points_to_edge(&p1, &p2);
    lefts.extend(l);
    rights.extend(r);
  }
  let lefts_set: HashSet<Point> = lefts
    .into_iter()
    .filter(|p| !perim.contains(p))
    .collect::<HashSet<_>>();
  let rights_set = rights
    .into_iter()
    .filter(|p| !perim.contains(p))
    .collect::<HashSet<_>>();
  let min_x: i64 = points.iter().map(|p| p.0).min().unwrap();
  let max_x = points.iter().map(|p| p.0).max().unwrap();
  let min_y = points.iter().map(|p| p.1).min().unwrap();
  let max_y = points.iter().map(|p| p.1).max().unwrap();

  let first = points[0];
  for y in (first.1 - 50)..=(first.1 + 50) {
    for x in (first.0 - 50)..=(first.0 + 50) {
      let point = (x, y);
      if points.contains(&point) {
        print!("#");
      } else if perim.contains(&point) {
        print!("X");
      } else if lefts_set.contains(&point) {
        print!("L");
      } else if rights_set.contains(&point) {
        print!("R");
      } else {
        print!(".");
      }
    }
    println!();
  }

  // let mut cache = &mut HashMap::new();
  // for y in 0..=max_y + 1 {
  //   for x in 0..=max_x + 1 {
  //     let point = (x, y);
  //     if is_inside_polygon(
  //       &point,
  //       min_x,
  //       &perim,
  //       &lefts_set,
  //       &rights_set,
  //       &mut cache,
  //       0,
  //     ) {
  //       print!("I")
  //     } else {
  //       print!(".")
  //     }
  //   }
  //   println!();
  // }
  // for x in min_x - 1..=max_x + 1 {
  //   is_on_perimeter_or_inside(&(x, 1), min_x, &perim, &lefts_set, &rights_set);
  // }
  println!("min_x: {:?}", min_x);
  println!("max_x: {:?}", max_x);
  println!("min_y: {:?}", min_y);
  println!("max_y: {:?}", max_y);

  let mut heap = heapify(&pairs);
  let hlen = heap.len() as f32;
  let mut so_far = 0f32;
  while heap.len() >= 1 {
    so_far += 1.0;
    let box_with_area = heap.pop().unwrap();
    println!(
      "considering {:?} with area {:?}, {:?}",
      box_with_area,
      box_with_area.area(),
      so_far / hlen * 100.0,
    );
    let BoxWithArea((ax, ay), (bx, by)) = box_with_area;
    let corners = &vec![(ax, ay), (bx, ay), (bx, by), (ax, by)];
    let box_perim = perimeter(corners);

    let point_outside_box = box_perim.iter().find(|p| lefts_set.contains(p));
    if let Some(point) = point_outside_box {
      // println!("point {:?} is outside the box", point);
    } else {
      println!("part 2 answer {:?}", box_with_area.area());
      break;
    }
  }
}

fn heapify(pairs: &Vec<(Point, Point)>) -> BinaryHeap<BoxWithArea> {
  pairs.iter().map(|(a, b)| BoxWithArea(*a, *b)).collect()
}

fn is_inside_polygon(
  point: &Point,
  min_x: i64,
  perim: &HashSet<Point>,
  lefts_set: &HashSet<Point>,
  rights_set: &HashSet<Point>,
  cache: &mut HashMap<Point, (i32, i32, i32, i32)>,
  area: i64,
) -> bool {
  if perim.contains(point) {
    return true;
  }
  let printing_enabled = false;
  let (lx, xl, rx, xr) = is_inside_polygon_helper(
    point,
    min_x,
    perim,
    lefts_set,
    rights_set,
    cache,
    printing_enabled,
  );
  (xl + xr) != (lx + rx)
}

fn is_inside_polygon_helper(
  curr: &Point,
  min_x: i64,
  perim: &HashSet<Point>,
  lefts_set: &HashSet<Point>,
  rights_set: &HashSet<Point>,
  cache: &mut HashMap<Point, (i32, i32, i32, i32)>,
  printing_enabled: bool,
) -> (i32, i32, i32, i32) {
  if curr.0 < min_x {
    if printing_enabled {
      println!("base case {:?}, returning 0, 0, 0, 0", curr);
    }
    return (0, 0, 0, 0);
  }
  let next = (curr.0 - 1, curr.1);
  let (mut lx, mut xl, mut xr, mut rx) = match cache.get(&curr) {
    None => {
      if printing_enabled {
        println!("curr {:?} not in cache, computing", curr);
      }
      is_inside_polygon_helper(
        &next,
        min_x,
        perim,
        lefts_set,
        rights_set,
        cache,
        printing_enabled,
      )
    }
    Some(cached) => return *cached,
  };
  if lefts_set.contains(&curr) && perim.contains(&next) {
    lx += 1;
  } else if perim.contains(&curr) && lefts_set.contains(&next) {
    xl += 1;
  } else if perim.contains(&curr) && rights_set.contains(&next) {
    xr += 1;
  } else if rights_set.contains(&curr) && perim.contains(&next) {
    rx += 1;
  }
  if printing_enabled {
    println!(
      "curr {:?}, next {:?} lx {:?}, xl {:?}, xr {:?}, rx {:?}",
      curr, next, lx, xl, xr, rx
    );
  }
  cache.insert(*curr, (lx, xl, xr, rx));
  (lx, xl, xr, rx)
}

fn adjacent_points_to_edge(p1: &Point, p2: &Point) -> (Vec<Point>, Vec<Point>) {
  let ((ax, ay), (bx, by)) = (*p1, *p2);
  assert_ne!(ax == bx, ay == by);
  let l_points: Vec<Point>;
  let r_points: Vec<Point>;
  if ax == bx {
    if ay < by {
      l_points = (ay..=by).map(|y| (ax + 1, y)).collect();
      r_points = (ay..=by).map(|y| (ax - 1, y)).collect();
    } else {
      // ay > by
      l_points = (by..=ay).map(|y| (ax - 1, y)).collect();
      r_points = (by..=ay).map(|y| (ax + 1, y)).collect();
    }
  } else if ay == by {
    if ax < bx {
      l_points = (ax..=bx).map(|x| (x, ay - 1)).collect();
      r_points = (ax..=bx).map(|x| (x, ay + 1)).collect();
    } else {
      l_points = (bx..=ax).map(|x| (x, ay + 1)).collect();
      r_points = (bx..=ax).map(|x| (x, ay - 1)).collect();
    }
  } else {
    unreachable!()
  }
  (l_points, r_points)
}

fn zip_with_next(points: &Vec<Point>) -> Vec<(Point, Point)> {
  let mut rotated = points.clone();
  rotated.rotate_left(1);
  points.iter().copied().zip(rotated).collect()
}

fn perimeter(corners: &Vec<Point>) -> Vec<Point> {
  let mut rotated = corners.clone();
  rotated.rotate_left(1);

  let with_next = corners.iter().zip(rotated).collect::<Vec<_>>();
  with_next
    .iter()
    .flat_map(|((ax, ay), (bx, by))| {
      let (ax, ay, bx, by) = (*ax, *ay, *bx, *by);
      if ax == bx {
        (min(ay, by)..=max(ay, by))
          .map(move |y| (ax, y))
          .collect::<Vec<_>>()
      } else if ay == by {
        (min(ax, bx)..=max(ax, bx))
          .map(move |x| (x, ay))
          .collect::<Vec<_>>()
      } else {
        unreachable!()
      }
    })
    .collect()
}

fn area(a: &Point, b: &Point) -> i64 {
  (a.0 - b.0 + 1).abs() * (a.1 - b.1 + 1).abs()
}

fn parse_input(input: &str) -> Vec<Point> {
  input
    .split('\n')
    .map(|line| {
      let [x, y] = line.split(",").collect::<Vec<_>>()[..] else {
        panic!("Invalid line: {:?}", line)
      };
      (
        x.parse::<i64>().expect("Failed to parse x coordinate"),
        y.parse::<i64>().expect("Failed to parse y coordinate"),
      )
    })
    .collect::<Vec<Point>>()
}
