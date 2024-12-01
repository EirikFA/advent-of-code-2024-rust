use crate::utils::read_lines;

use super::Solver;

pub struct Day1;

impl Solver for Day1 {
  type Input = (Vec<u32>, Vec<u32>);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => lines
        .flatten()
        .map(|line| {
          let nums: Vec<u32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
          (nums[0], nums[1])
        })
        .unzip(),
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(11.to_string())
  }

  fn part_1((left, right): &Self::Input) -> Self::Output1 {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.sort_unstable();
    right.sort_unstable();

    left
      .iter()
      .zip(right.iter())
      .fold(0, |acc, (n1, n2)| acc + n1.abs_diff(*n2))
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(31.to_string())
  }

  fn part_2((left, right): &Self::Input) -> Self::Output2 {
    let right_counts = right
      .iter()
      .fold(std::collections::HashMap::new(), |mut acc, n| {
        *acc.entry(*n).or_insert(0) += 1;
        acc
      });

    left
      .iter()
      .fold(0, |acc, n1| acc + n1 * right_counts.get(n1).unwrap_or(&0))
      .to_string()
  }
}
