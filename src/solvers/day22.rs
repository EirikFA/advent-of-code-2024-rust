use std::collections::HashMap;

use crate::utils::read_lines;

use super::Solver;

pub struct Day22 {}

impl Solver for Day22 {
  type Input = Vec<u64>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => lines.flatten().map(|line| line.parse().unwrap()).collect(),
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(37327623.to_string())
  }

  fn part_1(secrets: &Self::Input) -> Self::Output1 {
    secrets
      .iter()
      .map(|secret| (0..2000).fold(*secret, |acc, _| Day22::get_next(acc)))
      .sum::<u64>()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(23.to_string())
  }

  fn part_2(secrets: &Self::Input) -> Self::Output2 {
    let mut secret_seqs: Vec<HashMap<(i8, i8, i8, i8), i8>> = vec![HashMap::new(); secrets.len()];
    for (i, secret) in secrets.into_iter().enumerate() {
      let mut prices_diffs = Vec::new();
      let mut prev = *secret;
      for j in 0..2000 {
        let next = Day22::get_next(prev);
        let price = Day22::last_digit(next);
        let diff = price - Day22::last_digit(prev);
        prices_diffs.push((price, diff));
        prev = next;
        if j >= 3 {
          secret_seqs[i]
            .entry((
              prices_diffs[j - 3].1,
              prices_diffs[j - 2].1,
              prices_diffs[j - 1].1,
              prices_diffs[j].1,
            ))
            .or_insert(prices_diffs[j].0);
        }
      }
    }

    let mut seq_bananas = HashMap::new();
    for seqs in secret_seqs {
      for (seq, bananas) in seqs {
        seq_bananas
          .entry(seq)
          .and_modify(|e| *e += bananas as u32)
          .or_insert(bananas as u32);
      }
    }

    seq_bananas
      .iter()
      .max_by_key(|(_, &bananas)| bananas)
      .unwrap()
      .1
      .to_string()
  }
}

impl Day22 {
  fn last_digit(num: u64) -> i8 {
    (num % 10) as i8
  }

  fn get_next(secret: u64) -> u64 {
    let mut next = Self::mix_prune(secret * 64, secret);
    next = Self::mix_prune(next / 32, next);
    Self::mix_prune(next * 2048, next)
  }

  fn mix_prune(num: u64, secret: u64) -> u64 {
    return (num ^ secret) % 16777216;
  }
}
