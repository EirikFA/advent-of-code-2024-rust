use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

type Grid = HashMap<(isize, isize), char>;
type Region = HashSet<(isize, isize)>;

pub struct Day12 {}

impl Solver for Day12 {
  type Input = Grid;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut grid: HashMap<(isize, isize), char> = HashMap::new();
        for (y, line) in lines.flatten().enumerate() {
          for (x, plant) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), plant);
          }
        }
        grid
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(1930.to_string())
  }

  fn part_1(grid: &Self::Input) -> Self::Output1 {
    let mut cost = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for (x, y) in grid.keys() {
      if visited.contains(&(*x, *y)) {
        continue;
      }
      let mut region = Region::new();
      let perimeter = Self::make_region(grid, (*x, *y), &mut region);
      cost += perimeter * region.len();
      visited.extend(region);
    }
    cost.to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    // Some(1206.to_string())
    None
  }

  fn part_2(_regions: &Self::Input) -> Self::Output2 {
    1.to_string()
  }
}

impl Day12 {
  // DFS/flood fill
  fn make_region(grid: &Grid, (x, y): (isize, isize), region: &mut Region) -> usize {
    if region.contains(&(x, y)) {
      return 0;
    }
    region.insert((x, y));

    let neighbors = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    let mut perimeter = 0;
    for (nx, ny) in neighbors {
      let plant = grid.get(&(nx, ny));
      if plant.is_none() || plant.unwrap() != grid.get(&(x, y)).unwrap() {
        perimeter += 1;
        continue;
      }
      perimeter += Self::make_region(grid, (nx, ny), region);
    }
    perimeter
  }
}
