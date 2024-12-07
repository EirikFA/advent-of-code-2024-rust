use std::iter::repeat;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::read_lines;

use super::Solver;

type Equation = (u64, Vec<u64>);

pub struct Day7 {}

impl Solver for Day7 {
  type Input = Vec<Equation>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => lines
        .flatten()
        .map(|line| {
          let parts: Vec<&str> = line.split(": ").collect();
          let nums: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
          (parts[0].parse().unwrap(), nums)
        })
        .collect(),
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(3749.to_string())
  }

  fn part_1(equations: &Self::Input) -> Self::Output1 {
    // Parallelism improves this 5-10x as well, but it's already milliseconds so no big deal
    equations
      .par_iter()
      .filter(|eq| Self::try_operators(eq, false))
      .map(|(val, _)| val)
      .sum::<u64>()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(11387.to_string())
  }

  fn part_2(equations: &Self::Input) -> Self::Output2 {
    // Development build with no parallelism: ~9s
    // Release build with no parallelism: ~1.8s
    // Development build with parallelism: ~1.4s
    // Release build with parallelism: ~0.3s
    // I love Rust
    // TODO: Maybe we can do something clever? Like actually learning to solve the problem?
    equations
      .par_iter()
      .filter(|eq| Self::try_operators(eq, true))
      .map(|(val, _)| val)
      .sum::<u64>()
      .to_string()
  }
}

impl Day7 {
  fn try_operators((test_val, nums): &Equation, with_concat: bool) -> bool {
    let mut operators = vec!["+", "*"];
    if with_concat {
      operators.push("||");
    }

    let combinations = repeat(operators)
      .take(nums.len() - 1)
      .multi_cartesian_product();

    for operators in combinations {
      let mut val = nums[0] as u64;
      for (i, num) in nums.iter().skip(1).enumerate() {
        match operators[i] {
          "+" => val += *num as u64,
          "*" => val *= *num as u64,
          "||" => val = val * 10_u64.pow(num.to_string().len() as u32) + num,
          _ => panic!(),
        }
        if val > *test_val {
          break;
        }
      }
      if val == *test_val {
        return true;
      }
    }

    false
  }
}
