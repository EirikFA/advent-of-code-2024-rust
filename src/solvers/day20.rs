use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

type Position = (i32, i32);

pub struct Day20 {}

impl Solver for Day20 {
  type Input = (HashSet<Position>, Position, Position);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut track = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in lines.flatten().enumerate() {
          for (x, char) in line.chars().enumerate() {
            if char == '#' {
              continue;
            }

            let (x, y) = (x as i32, y as i32);
            if char == 'S' {
              start = (x, y);
            }
            if char == 'E' {
              end = (x, y);
            }
            track.insert((x, y));
          }
        }

        (track, start, end)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    None
  }

  fn part_1((track, start, end): &Self::Input) -> Self::Output1 {
    let costs = Self::find_path_costs(track, start, end);
    Self::find_cheats(track, start, end, &costs, costs[start], 100).to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2((_track, _start, _end): &Self::Input) -> Self::Output2 {
    // TODO
    panic!();
  }
}

impl Day20 {
  fn find_path_costs(
    track: &HashSet<Position>,
    start: &Position,
    end: &Position,
  ) -> HashMap<Position, i32> {
    let mut costs = HashMap::new();
    let (mut x, mut y) = end;
    let mut prev = *end;
    let mut cost = 1;
    while (x, y) != *start {
      let neighbours = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      for n in neighbours {
        if n != prev && track.contains(&n) {
          costs.insert(n, cost);
          prev = (x, y);
          (x, y) = n;
          cost += 1;
          break;
        }
      }
    }
    costs.insert(*end, 0);
    costs
  }

  fn find_cheats(
    track: &HashSet<Position>,
    start: &Position,
    end: &Position,
    costs: &HashMap<Position, i32>,
    max: i32,
    threshold: i32,
  ) -> u32 {
    let mut stack = vec![(start.clone(), 0)];
    let mut count = 0;

    while let Some((pos, cost)) = stack.pop() {
      if &pos == end {
        continue;
      }

      let (x, y) = pos;
      let neighbours = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      let neighbours_neighbours = vec![(x - 2, y), (x + 2, y), (x, y - 2), (x, y + 2)];

      for (i, n) in neighbours.iter().enumerate() {
        if track.contains(n) && costs[n] < costs[&pos] {
          stack.push((n.clone(), cost + 1));
        }

        let nn = neighbours_neighbours[i];
        if track.contains(&nn) {
          let saved = max - (cost + costs[&nn] + 2);
          if saved >= threshold {
            count += 1;
          }
        }
      }
    }

    count
  }
}
