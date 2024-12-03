use regex::Regex;

use super::Solver;

pub struct Day3 {}

impl Solver for Day3 {
  type Input = (Vec<(u32, u32)>, Vec<(u32, u32)>);

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    let mut do_operands: Vec<(u32, u32)> = Vec::new();
    let mut dont_operands: Vec<(u32, u32)> = Vec::new();

    let dos = input.split("do()");
    for do_str in dos {
      let donts: Vec<&str> = do_str.split("don't()").collect();
      do_operands.append(&mut Self::parse_operands(donts[0]));

      for dont_str in donts.iter().skip(1) {
        dont_operands.append(&mut Self::parse_operands(dont_str));
      }
    }

    (do_operands, dont_operands)
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(161.to_string())
  }

  fn part_1((do_ops, dont_ops): &Self::Input) -> Self::Output1 {
    (Self::calculate(do_ops) + Self::calculate(dont_ops)).to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(48.to_string())
  }

  fn part_2((do_ops, _): &Self::Input) -> Self::Output2 {
    Self::calculate(do_ops).to_string()
  }
}

impl Day3 {
  fn parse_operands(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

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

  fn calculate(ops: &Vec<(u32, u32)>) -> u32 {
    ops.iter().fold(0, |acc, (a, b)| acc + a * b)
  }
}
