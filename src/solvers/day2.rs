use crate::utils::read_lines;

use super::Solver;

pub struct Day2 {}

impl Solver for Day2 {
  type Input = Vec<Vec<u32>>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => lines
        .flatten()
        .map(|line| {
          line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
        })
        .collect(),
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(2.to_string())
  }

  fn part_1(reports: &Self::Input) -> Self::Output1 {
    reports
      .iter()
      .filter(|report| Self::report_safe(report))
      .count()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(4.to_string())
  }

  fn part_2(reports: &Self::Input) -> Self::Output2 {
    reports
      .iter()
      .filter(|report| Self::report_safe(report) || Self::report_safe_with_dampening(report))
      .count()
      .to_string()
  }
}

impl Day2 {
  fn report_safe_with_dampening(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
      let mut report = report.to_vec();
      report.remove(i);
      if Self::report_safe(&report) {
        return true;
      }
    }
    false
  }

  fn report_safe(report: &Vec<u32>) -> bool {
    let increasing = report[0] < report[1];
    report
      .windows(2)
      .all(|levels| Self::adjacent_levels_safe(levels, increasing))
  }

  fn adjacent_levels_safe(levels: &[u32], increasing: bool) -> bool {
    if let [l1, l2] = levels {
      let diff = l1.abs_diff(*l2);
      diff >= 1 && diff <= 3 && ((increasing && l1 < l2) || (!increasing && l1 > l2))
    } else {
      panic!("Invalid adjacent levels size");
    }
  }
}
