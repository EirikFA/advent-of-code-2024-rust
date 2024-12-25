use itertools::{Either, Itertools};
use regex::{Match, Regex};

use super::Solver;

type Heights = [u8; Day25::WIDTH];

pub struct Day25 {}

impl Solver for Day25 {
  type Input = (Vec<Heights>, Vec<Heights>);

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let regex = Regex::new(r"([.#]+)").unwrap();
    for schematic in (&regex.find_iter(input).chunks(Self::HEIGHT))
      .into_iter()
      .map(|chunk| chunk.collect::<Vec<_>>())
    {
      let (heights, is_lock) = Self::parse_schematic(schematic);
      if is_lock {
        locks.push(heights);
      } else {
        keys.push(heights);
      }
    }

    (locks, keys)
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(3.to_string())
  }

  fn part_1((locks, keys): &Self::Input) -> Self::Output1 {
    let mut fits = 0;
    for lock in locks {
      for key in keys {
        if lock
          .iter()
          .zip(key.iter())
          .all(|(l, k)| (l + k) as usize <= Self::HEIGHT - 2)
        {
          fits += 1;
        }
      }
    }
    fits.to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2((_locks, _keys): &Self::Input) -> Self::Output2 {
    0.to_string()
  }
}

impl Day25 {
  const WIDTH: usize = 5;
  const HEIGHT: usize = 7;

  fn parse_schematic(schematic: Vec<Match>) -> (Heights, bool) {
    let is_lock = schematic.get(0).unwrap().as_str() == "#".repeat(Self::WIDTH);
    let iterator = match is_lock {
      true => Either::Left(schematic.iter().skip(1)),
      false => Either::Right(schematic.iter().rev().skip(1)),
    };
    let mut heights = [0; 5];
    for line in iterator {
      for (j, char) in line.as_str().chars().enumerate() {
        if char == '#' {
          heights[j] += 1;
        }
      }
    }
    (heights, is_lock)
  }
}
