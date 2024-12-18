use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

use super::Solver;

type Position = (i32, i32);

pub struct Day18 {}

impl Solver for Day18 {
  type Input = Vec<Position>;

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    let regex = Regex::new(r"(\d+),(\d+)").unwrap();
    regex
      .captures_iter(input)
      .map(|captures| (captures[1].parse().unwrap(), captures[2].parse().unwrap()))
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    None
  }

  fn part_1(bytes: &Self::Input) -> Self::Output1 {
    let blocked = bytes.into_iter().take(Self::BYTE_COUNT).collect();
    let (distances, _) = Self::bfs(&blocked);
    distances[&(Self::GRID_SIZE, Self::GRID_SIZE)].to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2(bytes: &Self::Input) -> Self::Output2 {
    // Better algorithms? Nah, what about parallelism? (~1.2s to ~170ms) ( ͡° ͜ʖ ͡°)
    // let best = Mutex::new((usize::MAX, (0, 0)));
    // (Self::BYTE_COUNT..bytes.len())
    //   .into_par_iter()
    //   .for_each(|i| {
    //     let blocked = bytes.iter().take(i).cloned().collect();
    //     let (distances, _) = Self::bfs(&blocked);
    //     if distances.get(&(Self::GRID_SIZE, Self::GRID_SIZE)).is_none() {
    //       let mut best = best.lock().unwrap();
    //       if i < best.0 {
    //         let (x, y) = bytes[i - 1];
    //         *best = (i, (x, y));
    //       }
    //     }
    //   });
    // let (x, y) = best.into_inner().unwrap().1;
    // format!("{},{}", x, y)

    // But this is actually faster, and pretty simple
    // let mut path: HashSet<Position> = HashSet::new();
    // let mut blocked: HashSet<&(i32, i32)> = bytes.into_iter().take(Self::BYTE_COUNT).collect();
    // for i in Self::BYTE_COUNT..bytes.len() {
    //   if !path.is_empty() && !path.contains(&bytes[i - 1]) {
    //     continue;
    //   }

    //   blocked.insert(&bytes[i - 1]);
    //   let (distances, previous) = Self::bfs(&blocked);
    //   if distances.get(&(Self::GRID_SIZE, Self::GRID_SIZE)).is_none() {
    //     let (x, y) = bytes[i - 1];
    //     return format!("{},{}", x, y);
    //   }
    //   path = Self::get_path(&previous);
    // }

    // Binary search reigns supreme
    let mut left = Self::BYTE_COUNT;
    let mut right = bytes.len();
    while left < right {
      let mid = (left + right) / 2;
      let blocked = bytes.iter().take(mid + 1).collect();
      let (distances, _) = Self::bfs(&blocked);
      if distances.get(&(Self::GRID_SIZE, Self::GRID_SIZE)).is_none() {
        if left == mid {
          let (x, y) = bytes[mid];
          return format!("{},{}", x, y);
        }
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }

    panic!();
  }
}

impl Day18 {
  const BYTE_COUNT: usize = 1024;
  const GRID_SIZE: i32 = 70;

  fn bfs(blocked: &HashSet<&Position>) -> (HashMap<Position, u32>, HashMap<Position, Position>) {
    let mut distances: HashMap<Position, u32> = HashMap::new();
    let mut previous: HashMap<Position, Position> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<Position> = VecDeque::new();

    distances.insert((0, 0), 0);
    visited.insert((0, 0));
    queue.push_back((0, 0));

    while let Some((x, y)) = queue.pop_front() {
      let neighbours = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
      for (nx, ny) in neighbours {
        if nx < 0
          || ny < 0
          || nx > Self::GRID_SIZE
          || ny > Self::GRID_SIZE
          || blocked.contains(&(nx, ny))
          || visited.contains(&(nx, ny))
        {
          continue;
        }

        visited.insert((nx, ny));
        distances.insert((nx, ny), distances[&(x, y)] + 1);
        previous.insert((nx, ny), (x, y));
        queue.push_back((nx, ny));
      }
    }
    (distances, previous)
  }

  // fn get_path(previous: &HashMap<Position, Position>) -> HashSet<Position> {
  //   let mut path = HashSet::new();
  //   let mut pos = (Self::GRID_SIZE, Self::GRID_SIZE);
  //   while let Some(prev) = previous.get(&pos) {
  //     path.insert(pos);
  //     pos = *prev;
  //   }
  //   path
  // }
}
