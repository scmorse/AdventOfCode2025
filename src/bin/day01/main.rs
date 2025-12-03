mod dial_state;

use dial_state::DialState;
use num_integer::div_rem;
use std::fs;
use tailcall::tailcall;

// https://adventofcode.com/2025/day/1
fn main() {
  let filename = "src/bin/day01/input-real-ignore.txt";
  let input = fs::read_to_string(filename).expect("Failed to read input file");
  let numbers: Vec<i32> = input.lines().map(parse_line).collect();

  // Part 1
  let part_1_answer = count_resting_at_zero(DialState::new(), &numbers).zero_position_count;
  check_part_1_answer(filename, part_1_answer);

  // Part 2
  let part_2_answer = count_passing_zero(DialState::new(), &numbers).zero_position_count;
  check_part_2_answer(filename, part_2_answer);
}

#[allow(unreachable_code)]
#[tailcall]
fn count_passing_zero(state: DialState, numbers: &[i32]) -> DialState {
  let Some((&first, rest)) = numbers.split_first() else {
    return state;
  };
  let block_size = first.abs().cast_unsigned();
  let (quotient, remainder) = div_rem(block_size, 100);
  count_passing_zero(
    DialState {
      current_dial_position: (state.current_dial_position.cast_signed() + first)
        .rem_euclid(100)
        .cast_unsigned(),
      zero_position_count: state.zero_position_count + quotient +
        // This is where the real logic happens. It's based on the idea that a block of N
        // contiguous numbers must always contain `quotient` multiples of 100. Whether it will
        // contain one extra multiple of 100 depends on how close it is to the nearest multiple
        // in the direction of the turn.
        // Example: block_size = 103, current_dial_position = 99 => will cross 2 multiples of 100
        // Example: block_size = 103, current_dial_position = 80 => will cross 1 multiple of 100
        match (first, state.current_dial_position) {
          (n, pos) if n > 0 && pos >= 100 - remainder => 1,
          (n, pos) if n < 0 && (1..=remainder).contains(&pos) => 1,
          _ => 0,
        },
    },
    rest,
  );
}

#[allow(unreachable_code)]
#[tailcall]
fn count_resting_at_zero(state: DialState, numbers: &[i32]) -> DialState {
  let Some((&first, rest)) = numbers.split_first() else {
    return state;
  };
  let new_position = (state.current_dial_position.cast_signed() + first)
    .rem_euclid(100)
    .cast_unsigned();
  count_resting_at_zero(
    DialState {
      current_dial_position: new_position,
      zero_position_count: if new_position == 0 {
        state.zero_position_count + 1
      } else {
        state.zero_position_count
      },
    },
    rest,
  );
}

fn check_part_1_answer(filename: &str, zero_position_count: u32) {
  println!("Part 1 answer: {0}", zero_position_count);
  match filename {
    "src/bin/day01/input-fake-ignore.txt" => assert_eq!(zero_position_count, 3),
    "src/bin/day01/input-real-ignore.txt" => assert_eq!(zero_position_count, 1089),
    _ => panic!("Unexpected filename: {}", filename),
  }
}

fn check_part_2_answer(filename: &str, zero_position_count: u32) {
  println!("Part 2 answer: {0}", zero_position_count);
  match filename {
    "src/bin/day01/input-fake-ignore.txt" => assert_eq!(zero_position_count, 6),
    "src/bin/day01/input-real-ignore.txt" => assert_eq!(zero_position_count, 6530),
    _ => panic!("Unexpected filename: {filename}"),
  }
}

fn parse_line(line: &str) -> i32 {
  let first_char: char = line.chars().next().expect("Empty line");
  let number: i32 = line[1..].parse().expect("Invalid number");

  match first_char {
    'R' => number,
    'L' => -number,
    _ => panic!("Line must start with R or L: {}", line),
  }
}
