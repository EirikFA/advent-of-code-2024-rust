use itertools::Itertools;
use regex::Regex;

use crate::utils::read_lines;

use super::Solver;

#[derive(Debug, Clone)]
pub struct ClawMachine {
  a_button: (i64, i64),
  b_button: (i64, i64),
  prize: (i64, i64),
}

pub struct Day13 {}

impl Solver for Day13 {
  type Input = Vec<ClawMachine>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let regex = Regex::new(r"X[=+](\d+).*Y[=+](\d+)").unwrap();
        lines
          .flatten()
          .chunks(4)
          .into_iter()
          .map(|lines| {
            let text = lines.collect::<Vec<String>>().join("\n");
            let nums = Day13::parse_x_y(&text, &regex);
            ClawMachine {
              a_button: nums[0],
              b_button: nums[1],
              prize: nums[2],
            }
          })
          .collect()
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(480.to_string())
  }

  fn part_1(machines: &Self::Input) -> Self::Output1 {
    Day13::sum_costs(machines).to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2(machines: &Self::Input) -> Self::Output2 {
    let mut machines = machines.clone();
    for machine in machines.iter_mut() {
      machine.prize.0 = machine.prize.0 + 10000000000000;
      machine.prize.1 = machine.prize.1 + 10000000000000;
    }
    Day13::sum_costs(&machines).to_string()
  }
}

impl Day13 {
  fn parse_x_y(input: &str, regex: &Regex) -> Vec<(i64, i64)> {
    regex
      .captures_iter(input)
      .map(|captures| {
        (
          captures.get(1).unwrap().as_str().parse().unwrap(),
          captures.get(2).unwrap().as_str().parse().unwrap(),
        )
      })
      .collect()
  }

  fn sum_costs(machines: &Vec<ClawMachine>) -> i64 {
    machines.iter().map(Day13::calculate_cost).sum()
  }

  fn calculate_cost(machine: &ClawMachine) -> i64 {
    match Day13::calculate_moves(machine) {
      Some((a, b)) => 3 * a + b,
      None => 0,
    }
  }

  fn calculate_moves(
    ClawMachine {
      a_button: (a_x, a_y),
      b_button: (b_x, b_y),
      prize: (p_x, p_y),
    }: &ClawMachine,
  ) -> Option<(i64, i64)> {
    let a_nom = p_x * b_y - p_y * b_x;
    let a_denom = a_x * b_y - a_y * b_x;
    if a_nom % a_denom != 0 {
      return None;
    }
    let a = a_nom / a_denom;

    let b_nom = p_y - a_y * a;
    if b_nom % b_y != 0 {
      return None;
    }
    let b = (p_y - a_y * a) / b_y;

    Some((a as i64, b as i64))
  }
}
