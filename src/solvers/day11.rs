use std::collections::HashMap;

use super::Solver;

pub struct Day11 {}

impl Solver for Day11 {
  type Input = Vec<u64>;

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    input
      .split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(55312.to_string())
  }

  fn part_1(stones: &Self::Input) -> Self::Output1 {
    (0..25)
      .fold(stones.clone(), |stones, _| Day11::blink(&stones))
      .len()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2(stones: &Self::Input) -> Self::Output2 {
    let mut stones_map: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
      *stones_map.entry(*stone).or_insert(0) += 1;
    }
    (0..75)
      .fold(stones_map, |stones, _| Day11::blink_faster(stones))
      .values()
      .sum::<u64>()
      .to_string()
  }
}

impl Day11 {
  fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();
    for stone in stones {
      if *stone == 0 {
        new_stones.push(1);
      } else {
        let digits = stone.to_string();
        let len = digits.len();
        if len % 2 == 0 {
          new_stones.push(digits[..len / 2].parse().unwrap());
          new_stones.push(digits[len / 2..].parse().unwrap());
        } else {
          new_stones.push(stone * 2024);
        }
      }
    }
    new_stones
  }

  fn blink_faster(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (stone, count) in stones {
      if stone == 0 {
        *new_stones.entry(1).or_insert(0) += count;
      } else {
        let digits = stone.to_string();
        let len = digits.len();
        if len % 2 == 0 {
          let stone_1 = digits[..len / 2].parse().unwrap();
          let stone_2 = digits[len / 2..].parse().unwrap();
          *new_stones.entry(stone_1).or_insert(0) += count;
          *new_stones.entry(stone_2).or_insert(0) += count;
        } else {
          *new_stones.entry(stone * 2024).or_insert(0) += count;
        }
      }
    }
    new_stones
  }
}
