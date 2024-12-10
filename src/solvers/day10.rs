use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::read_lines;

use super::Solver;

#[derive(Debug)]
pub struct Trailhead(usize, usize);

pub struct Day10 {}

impl Solver for Day10 {
  type Input = (Vec<Trailhead>, Vec<Vec<u32>>);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut trailheads: Vec<Trailhead> = Vec::new();
        let mut points: Vec<Vec<u32>> = Vec::new();

        for (y, line) in lines.flatten().enumerate() {
          points.push(Vec::new());
          for (x, char) in line.chars().enumerate() {
            let height: u32 = char.to_digit(10).unwrap();
            if height == 0 {
              trailheads.push(Trailhead(x, y));
            }
            points[y].push(height);
          }
        }
        (trailheads, points)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(36.to_string())
  }

  fn part_1((trailheads, points): &Self::Input) -> Self::Output1 {
    // About 2x-3x faster in parallel
    trailheads
      .par_iter()
      .map(|t| Self::dfs(t, points, true))
      .sum::<usize>()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(81.to_string())
  }

  fn part_2((trailheads, points): &Self::Input) -> Self::Output2 {
    trailheads
      .par_iter()
      .map(|t| Self::dfs(t, points, false))
      .sum::<usize>()
      .to_string()
  }
}

impl Day10 {
  fn dfs(trailhead: &Trailhead, points: &Vec<Vec<u32>>, visited_enabled: bool) -> usize {
    let visited = if visited_enabled {
      &mut Some(HashSet::new())
    } else {
      &mut None
    };

    let mut trails = 0;
    Self::dfs_visit((trailhead.0, trailhead.1), points, visited, &mut trails);
    trails
  }

  fn dfs_visit(
    point: (usize, usize),
    points: &Vec<Vec<u32>>,
    visited: &mut Option<HashSet<(usize, usize)>>,
    trails: &mut usize,
  ) {
    // Could improve efficiency by keeping track of how many trails we reached from the point we already visited
    // to avoid retracing the same trail (but with a different trail until now),
    // but it's definitely not slow as is
    if let Some(visited) = visited {
      if visited.contains(&point) {
        return;
      }
      visited.insert(point);
    }

    let (x, y) = point;
    let height = points[y][x];
    if height == 9 {
      *trails += 1;
    }

    let mut neighbors = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
      neighbors.push((x - 1, y));
    }
    if y > 0 {
      neighbors.push((x, y - 1));
    }

    for (x, y) in neighbors {
      if x >= points[0].len() || y >= points.len() {
        continue;
      }
      let neighbor_height = points[y][x];
      if neighbor_height == height + 1 {
        Self::dfs_visit((x, y), points, visited, trails);
      }
    }
  }
}
