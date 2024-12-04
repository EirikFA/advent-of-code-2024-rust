use crate::utils::read_lines;

use super::Solver;

pub struct Day4 {}

impl Solver for Day4 {
  type Input = Vec<Vec<char>>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => lines.flatten().map(|line| line.chars().collect()).collect(),
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(18.to_string())
  }

  fn part_1(chars: &Self::Input) -> Self::Output1 {
    let mut count = 0;

    for y in 0..chars.len() {
      for x in 0..chars[0].len() {
        if chars[y][x] != 'X' {
          continue;
        }

        // Not ideal, but pretty straightforward

        let x_end = x < chars[0].len() - 3;
        let x_start = x >= 3;
        let y_end = y < chars.len() - 3;
        let y_start = y >= 3;

        // Right/forward
        if x_end && chars[y][x + 1] == 'M' && chars[y][x + 2] == 'A' && chars[y][x + 3] == 'S' {
          count += 1;
        }

        // Left/backward
        if x_start && chars[y][x - 1] == 'M' && chars[y][x - 2] == 'A' && chars[y][x - 3] == 'S' {
          count += 1;
        }

        // Down
        if y_end && chars[y + 1][x] == 'M' && chars[y + 2][x] == 'A' && chars[y + 3][x] == 'S' {
          count += 1;
        }

        // Up
        if y_start && chars[y - 1][x] == 'M' && chars[y - 2][x] == 'A' && chars[y - 3][x] == 'S' {
          count += 1;
        }

        // Diagonal down right
        if x_end
          && y_end
          && chars[y + 1][x + 1] == 'M'
          && chars[y + 2][x + 2] == 'A'
          && chars[y + 3][x + 3] == 'S'
        {
          count += 1;
        }

        // Diagonal down left
        if x_start
          && y_end
          && chars[y + 1][x - 1] == 'M'
          && chars[y + 2][x - 2] == 'A'
          && chars[y + 3][x - 3] == 'S'
        {
          count += 1;
        }

        // Diagonal up right
        if x_end
          && y_start
          && chars[y - 1][x + 1] == 'M'
          && chars[y - 2][x + 2] == 'A'
          && chars[y - 3][x + 3] == 'S'
        {
          count += 1;
        }

        // Diagonal up left
        if x_start
          && y_start
          && chars[y - 1][x - 1] == 'M'
          && chars[y - 2][x - 2] == 'A'
          && chars[y - 3][x - 3] == 'S'
        {
          count += 1;
        }
      }
    }

    count.to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(9.to_string())
  }

  fn part_2(chars: &Self::Input) -> Self::Output2 {
    let mut count = 0;

    for y in 0..chars.len() {
      for x in 0..chars[0].len() {
        if x > chars[0].len() - 3 || y > chars.len() - 3 || chars[y + 1][x + 1] != 'A' {
          continue;
        }

        let right_diag = (chars[y][x] == 'M' && chars[y + 2][x + 2] == 'S')
          || (chars[y][x] == 'S' && chars[y + 2][x + 2] == 'M');
        let left_diag = (chars[y][x + 2] == 'M' && chars[y + 2][x] == 'S')
          || (chars[y][x + 2] == 'S' && chars[y + 2][x] == 'M');

        if right_diag && left_diag {
          count += 1;
        }
      }
    }

    count.to_string()
  }
}
