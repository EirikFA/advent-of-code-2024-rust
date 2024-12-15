use std::collections::HashSet;

use crate::utils::read_lines;

use super::Solver;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
  Empty,
  Wall,
  LeftBox,
  RightBox,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

type Position = (usize, usize);
type Map = Vec<Vec<Object>>;

pub struct Day15 {}

impl Solver for Day15 {
  type Input = (Position, Map, Vec<Direction>);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut robot = (0, 0);
        let mut map: Map = Vec::new();
        let mut moves: Vec<Direction> = Vec::new();
        let mut parsing_map = true;

        for (y, line) in lines.flatten().enumerate() {
          if line.is_empty() {
            parsing_map = false;
            continue;
          }

          if parsing_map {
            map.push(
              line
                .chars()
                .enumerate()
                .map(|(x, char)| match char {
                  '.' => Object::Empty,
                  '#' => Object::Wall,
                  'O' => Object::LeftBox,
                  '@' => {
                    robot = (x, y);
                    Object::Empty
                  }
                  _ => panic!(),
                })
                .collect(),
            );
          } else {
            for c in line.chars() {
              moves.push(match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!(),
              });
            }
          }
        }

        (robot, map, moves)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(10092.to_string())
  }

  fn part_1((robot, map, moves): &Self::Input) -> Self::Output1 {
    let mut map = map.clone();
    let mut robot = robot.clone();
    for dir in moves {
      Self::move_robot(&mut robot, dir, &mut map, false);
    }

    Self::sum_gps(&map).to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(9021.to_string())
  }

  fn part_2((robot, map, moves): &Self::Input) -> Self::Output2 {
    let mut new_map: Map = map
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|object| {
            if *object == Object::LeftBox {
              vec![Object::LeftBox, Object::RightBox]
            } else {
              vec![*object, *object]
            }
          })
          .flatten()
          .collect()
      })
      .collect();
    let mut robot = (robot.0 * 2, robot.1);
    for dir in moves {
      Self::move_robot(&mut robot, dir, &mut new_map, true);
    }

    Self::sum_gps(&new_map).to_string()
  }
}

impl Day15 {
  fn move_robot(robot: &mut Position, direction: &Direction, map: &mut Map, part_2: bool) {
    let (nx, ny) = Self::new_position(robot, direction);
    if map[ny][nx] == Object::Wall {
      return;
    }

    if map[ny][nx] == Object::Empty {
      *robot = (nx, ny);
    } else {
      if part_2 && (direction == &Direction::Up || direction == &Direction::Down) {
        let mut boxes: HashSet<usize> = HashSet::new();
        boxes.insert(nx);
        if map[ny][nx] == Object::LeftBox {
          boxes.insert(nx + 1);
        } else {
          boxes.insert(nx - 1);
        }

        let moved = Self::move_box_multiple(ny, &boxes, direction, map);
        if moved {
          *robot = (nx, ny);
        }
      } else {
        let moved = Self::move_box(&(nx, ny), direction, map);
        if moved {
          *robot = (nx, ny);
        }
      }
    }
  }

  fn move_box(pos: &Position, direction: &Direction, map: &mut Map) -> bool {
    let (nx, ny) = Self::new_position(pos, direction);
    if map[ny][nx] == Object::Wall {
      return false;
    }

    if map[ny][nx] == Object::Empty || Self::move_box(&(nx, ny), direction, map) {
      map[ny][nx] = map[pos.1][pos.0];
      map[pos.1][pos.0] = Object::Empty;
      return true;
    }

    false
  }

  fn move_box_multiple(
    y: usize,
    boxes: &HashSet<usize>,
    direction: &Direction,
    map: &mut Map,
  ) -> bool {
    if boxes.is_empty() {
      return true;
    }

    let ny = if direction == &Direction::Up {
      y - 1
    } else {
      y + 1
    };
    let mut new_row: HashSet<usize> = HashSet::new();
    for x in boxes {
      if map[ny][*x] == Object::Wall {
        return false;
      }
      if map[ny][*x] == Object::Empty {
        continue;
      }

      new_row.insert(*x);
      if map[ny][*x] == Object::LeftBox {
        new_row.insert(*x + 1);
      } else {
        new_row.insert(*x - 1);
      }
    }

    let moved = Self::move_box_multiple(ny, &new_row, direction, map);
    if moved {
      for x in boxes {
        map[ny][*x] = map[y][*x];
        map[y][*x] = Object::Empty;
      }
    }
    moved
  }

  fn new_position((x, y): &Position, direction: &Direction) -> Position {
    match direction {
      Direction::Up => (*x, *y - 1),
      Direction::Down => (*x, *y + 1),
      Direction::Left => (*x - 1, *y),
      Direction::Right => (*x + 1, *y),
    }
  }

  fn sum_gps(map: &Map) -> usize {
    map.iter().enumerate().fold(0, |sum, (y, row)| {
      row.iter().enumerate().fold(sum, |row_sum, (x, object)| {
        if *object == Object::LeftBox {
          row_sum + 100 * y + x
        } else {
          row_sum
        }
      })
    })
  }
}
