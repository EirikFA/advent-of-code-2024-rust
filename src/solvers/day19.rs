use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

pub struct Day19 {}

impl Solver for Day19 {
  type Input = (HashSet<String>, Vec<String>);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let lines: Vec<String> = lines.flatten().collect();
        let towels = lines[0].split(", ").map(|t| t.to_string()).collect();

        (towels, lines[2..].to_vec())
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(6.to_string())
  }

  fn part_1((towels, designs): &Self::Input) -> Self::Output1 {
    designs
      .iter()
      .filter(|d| Self::is_valid_design(d, towels))
      .count()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(16.to_string())
  }

  fn part_2((towels, designs): &Self::Input) -> Self::Output2 {
    let mut memo = HashMap::new();
    designs
      .iter()
      .map(|d| Self::valid_arrangements(d, towels, &mut memo))
      .sum::<u64>()
      .to_string()
  }
}

impl Day19 {
  fn is_valid_design(design: &str, towels: &HashSet<String>) -> bool {
    if design.is_empty() {
      return true;
    }

    for i in (0..design.len()).rev() {
      if towels.contains(&design[..=i]) {
        let valid = Self::is_valid_design(&design[(i + 1)..], towels);
        if valid {
          return true;
        }
      }
    }

    false
  }

  fn valid_arrangements(
    design: &str,
    towels: &HashSet<String>,
    memo: &mut HashMap<String, u64>,
  ) -> u64 {
    if design.is_empty() {
      return 1;
    }

    if let Some(&count) = memo.get(design) {
      return count;
    }

    let mut count = 0;
    for i in (0..design.len()).rev() {
      if towels.contains(&design[..=i]) {
        count += Self::valid_arrangements(&design[(i + 1)..], towels, memo);
      }
    }
    memo.insert(design.to_string(), count);
    count
  }
}
