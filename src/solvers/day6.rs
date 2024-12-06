use std::collections::HashSet;

use crate::utils::read_lines;

use super::Solver;

type Obstacles = Vec<HashSet<usize>>;

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position(usize, usize);

pub struct Day6 {}

impl Solver for Day6 {
  type Input = (Position, Obstacles, usize);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut obstacles: Obstacles = vec![];
        let mut guard_pos: Option<Position> = None;
        let mut width: usize = 0;

        for (y, line) in lines.flatten().enumerate() {
          let mut row = HashSet::new();
          if width == 0 {
            width = line.len();
          }

          for (x, char) in line.chars().enumerate() {
            match char {
              '#' => drop(row.insert(x)),
              '^' => guard_pos = Some(Position(x, y)),
              _ => (),
            }
          }

          obstacles.push(row);
        }

        (guard_pos.unwrap(), obstacles, width)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(41.to_string())
  }

  fn part_1((pos, obstacles, width): &Self::Input) -> Self::Output1 {
    let mut direction = Direction::Up;
    let mut pos = Position(pos.0, pos.1);
    let mut positions: HashSet<Position> = HashSet::new();

    loop {
      let new_pos = Self::encounter_obstacle(&pos, obstacles, &direction, *width);
      let x_range = Self::get_pos_iterator(pos.0, new_pos.0);
      let y_range = Self::get_pos_iterator(pos.1, new_pos.1);

      for x in x_range {
        positions.insert(Position(x, pos.1));
      }
      for y in y_range {
        positions.insert(Position(pos.0, y));
      }

      pos = new_pos;
      if pos.0 == 0 || pos.0 == *width - 1 || pos.1 == 0 || pos.1 == obstacles.len() - 1 {
        break;
      }

      direction = Self::rotate(&direction);
    }

    positions.len().to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2((_pos, _obstacles, _width): &Self::Input) -> Self::Output2 {
    // TODO
    0.to_string()
  }
}

impl Day6 {
  fn encounter_obstacle(
    pos: &Position,
    obstacles: &Obstacles,
    direction: &Direction,
    width: usize,
  ) -> Position {
    match direction {
      Direction::Up => {
        for y in (0..pos.1).rev() {
          if obstacles[y].contains(&pos.0) {
            return Position(pos.0, y + 1);
          }
        }
        return Position(pos.0, 0);
      }
      Direction::Down => {
        for y in pos.1..obstacles.len() {
          if obstacles[y].contains(&pos.0) {
            return Position(pos.0, y - 1);
          }
        }
        return Position(pos.0, obstacles.len() - 1);
      }
      Direction::Left => {
        for x in (0..pos.0).rev() {
          if obstacles[pos.1].contains(&x) {
            return Position(x + 1, pos.1);
          }
        }
        return Position(0, pos.1);
      }
      Direction::Right => {
        for x in pos.0..width {
          if obstacles[pos.1].contains(&x) {
            return Position(x - 1, pos.1);
          }
        }
        return Position(width - 1, pos.1);
      }
    };
  }

  fn get_pos_iterator(from: usize, to: usize) -> Box<dyn Iterator<Item = usize>> {
    if from < to {
      Box::new(from..=to)
    } else {
      Box::new((to..=from).rev())
    }
  }

  fn rotate(direction: &Direction) -> Direction {
    match direction {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }
}
