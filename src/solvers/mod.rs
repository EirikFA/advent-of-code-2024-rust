use std::{fmt::Display, fs::read_to_string};

pub mod day1;
pub mod day2;
pub mod day3;

pub trait Solver {
  type Input;
  type Output1: Display;
  type Output2: Display;

  fn parse(input: &str, path: &str) -> Self::Input;

  fn part_1_test_output() -> Option<Self::Output1>;
  fn part_1(input: &Self::Input) -> Self::Output1;

  fn part_2_test_output() -> Option<Self::Output2>;
  fn part_2(input: &Self::Input) -> Self::Output2;

  fn parse_file(path: &str) -> Self::Input {
    match read_to_string(path) {
      Ok(input) => Self::parse(&input, path),
      Err(error) => panic!("Error reading input file: {}", error),
    }
  }

  fn test(day_path: &str) {
    let part_1_expected = Self::part_1_test_output();
    if let Some(part_1_expected) = part_1_expected {
      let part_1_input = Self::parse_file(&format!("{}/part_1_test.txt", day_path));
      assert_eq!(
        Self::part_1(&part_1_input).to_string(),
        part_1_expected.to_string()
      );
    }

    let part_2_expected = Self::part_2_test_output();
    if let Some(part_2_expected) = part_2_expected {
      let part_2_input = Self::parse_file(&format!("{}/part_2_test.txt", day_path));
      assert_eq!(
        Self::part_2(&part_2_input).to_string(),
        part_2_expected.to_string()
      );
    }
  }

  fn run(day_path: &str) {
    Self::test(day_path);

    let path = format!("{}/input.txt", day_path);
    let input = Self::parse_file(&path);
    println!("Part 1: {}", Self::part_1(&input));
    println!("Part 2: {}", Self::part_2(&input));
  }
}