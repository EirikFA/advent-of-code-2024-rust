use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

type Position = (usize, usize);

#[derive(PartialEq, Eq)]
struct ReachableTile {
  position: Position,
  cost: usize,
  direction: Direction,
  path: Vec<Position>,
}

impl Ord for ReachableTile {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other
      .cost
      .cmp(&self.cost)
      .then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for ReachableTile {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

type Maze = HashSet<Position>;

pub struct Day16 {}

impl Solver for Day16 {
  type Input = (Maze, Position, Position);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut maze: Maze = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in lines.flatten().enumerate() {
          if width == 0 {
            width = line.len();
          }
          height += 1;

          for (x, char) in line.chars().enumerate() {
            if char != '#' {
              maze.insert((x, y));
            }
          }
        }

        (maze, (1, height - 2), (width - 2, 1))
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(11048.to_string())
  }

  // Djikstra
  fn part_1((maze, start, end): &Self::Input) -> Self::Output1 {
    let mut costs: HashMap<(Position, Direction), usize> = HashMap::new();
    costs.insert((*start, Direction::Right), 0);

    let mut heap = BinaryHeap::new();
    heap.push(ReachableTile {
      position: *start,
      cost: 0,
      direction: Direction::Right,
      path: vec![],
    });

    while let Some(tile) = heap.pop() {
      for neighbour in Self::find_neighbours(maze, &tile) {
        let ReachableTile {
          position,
          direction,
          cost,
          ..
        } = neighbour;

        if *costs.get(&(position, direction)).unwrap_or(&usize::MAX) > cost {
          costs.insert((position, direction), cost);
          heap.push(neighbour);
        }
      }
    }

    let end_costs = vec![
      costs.get(&(*end, Direction::Up)).unwrap_or(&usize::MAX),
      costs.get(&(*end, Direction::Right)).unwrap_or(&usize::MAX),
    ];
    end_costs.iter().min().unwrap().to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(64.to_string())
  }

  fn part_2((maze, start, end): &Self::Input) -> Self::Output2 {
    let mut costs: HashMap<(Position, Direction), usize> = HashMap::new();
    costs.insert((*start, Direction::Right), 0);

    let mut heap = BinaryHeap::new();
    heap.push(ReachableTile {
      position: *start,
      cost: 0,
      direction: Direction::Right,
      path: vec![*start],
    });

    let best_cost = Self::part_1(&(maze.clone(), *start, *end))
      .parse::<usize>()
      .unwrap();
    let mut best_paths_tiles: HashSet<Position> = HashSet::new();

    while let Some(tile) = heap.pop() {
      if tile.position == *end && tile.cost == best_cost {
        for position in tile.path {
          best_paths_tiles.insert(position);
        }
        continue;
      }

      for neighbour in Self::find_neighbours(maze, &tile) {
        let ReachableTile {
          position,
          direction,
          cost,
          ..
        } = neighbour;

        if *costs.get(&(position, direction)).unwrap_or(&usize::MAX) >= cost {
          heap.push(neighbour);
          costs.insert((position, direction), cost);
        }
      }
    }

    best_paths_tiles.insert(*end);
    best_paths_tiles.len().to_string()
  }
}

impl Day16 {
  fn find_neighbours(
    maze: &Maze,
    ReachableTile {
      position,
      direction,
      cost,
      path,
    }: &ReachableTile,
  ) -> Vec<ReachableTile> {
    let (x, y) = position;
    let mut neighbours: Vec<ReachableTile> = Vec::new();
    let directions = [
      (Direction::Up, (0, -1)),
      (Direction::Down, (0, 1)),
      (Direction::Left, (-1, 0)),
      (Direction::Right, (1, 0)),
    ];

    for &(new_dir, (dx, dy)) in &directions {
      let new_pos = ((*x as isize + dx) as usize, (*y as isize + dy) as usize);
      if maze.contains(&new_pos) {
        let mut path = path.clone();
        path.push(*position);

        neighbours.push(ReachableTile {
          position: new_pos,
          cost: if new_dir == *direction {
            cost + 1
          } else {
            cost + 1001
          },
          direction: new_dir,
          path,
        });
      }
    }
    neighbours
  }
}
