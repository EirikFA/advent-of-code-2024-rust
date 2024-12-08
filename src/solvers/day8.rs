use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point(i32, i32);

impl Point {
  fn inside_map(&self, width: u32, height: u32) -> bool {
    self.0 >= 0 && self.0 < width as i32 && self.1 >= 0 && self.1 < height as i32
  }
}

pub struct Day8 {}

impl Solver for Day8 {
  type Input = (HashMap<char, Vec<Point>>, u32, u32);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in lines.flatten().enumerate() {
          if width == 0 {
            width = line.len() as u32;
          }
          height += 1;

          for (x, char) in line.chars().enumerate() {
            if char == '.' {
              continue;
            }
            antennas
              .entry(char)
              .or_insert(Vec::new())
              .push(Point(x as i32, y as i32));
          }
        }
        (antennas, width, height)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(14.to_string())
  }

  fn part_1((antennas, w, h): &Self::Input) -> Self::Output1 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, antennas) in antennas {
      for (i, a_1) in antennas.iter().enumerate() {
        for a_2 in antennas.iter().skip(i + 1) {
          let dx = (a_1.0 as i32 - a_2.0 as i32).abs();
          let dy = (a_1.1 as i32 - a_2.1 as i32).abs();

          let (x_1, x_2) = if a_1.0 < a_2.0 {
            (a_1.0 - dx, a_2.0 + dx)
          } else {
            (a_1.0 + dx, a_2.0 - dx)
          };
          let (y_1, y_2) = if a_1.1 < a_2.1 {
            (a_1.1 - dy, a_2.1 + dy)
          } else {
            (a_1.1 + dy, a_2.1 - dy)
          };

          let antinode_1 = Point(x_1, y_1);
          let antinode_2 = Point(x_2, y_2);
          if antinode_1.inside_map(*w, *h) {
            antinodes.insert(antinode_1);
          }
          if antinode_2.inside_map(*w, *h) {
            antinodes.insert(antinode_2);
          }
        }
      }
    }

    antinodes.len().to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(34.to_string())
  }

  fn part_2((antennas, w, h): &Self::Input) -> Self::Output2 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, antennas) in antennas {
      for (i, a_1) in antennas.iter().enumerate() {
        for a_2 in antennas.iter().skip(i + 1) {
          let dx = (a_1.0 as i32 - a_2.0 as i32).abs();
          let dy = (a_1.1 as i32 - a_2.1 as i32).abs();

          let (mut x, mut y) = (a_1.0, a_1.1);
          while Point(x, y).inside_map(*w, *h) {
            antinodes.insert(Point(x, y));
            x = if a_1.0 < a_2.0 { x - dx } else { x + dx };
            y = if a_1.1 < a_2.1 { y - dy } else { y + dy };
          }

          (x, y) = (a_2.0, a_2.1);
          while Point(x, y).inside_map(*w, *h) {
            antinodes.insert(Point(x, y));
            x = if a_2.0 < a_1.0 { x - dx } else { x + dx };
            y = if a_2.1 < a_1.1 { y - dy } else { y + dy };
          }
        }
      }
    }

    antinodes.len().to_string()
  }
}

impl Day8 {}
