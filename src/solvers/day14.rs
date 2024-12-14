use std::collections::HashSet;

use image::{ImageBuffer, Rgb};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use super::Solver;

#[derive(Debug, Clone)]
pub struct Robot {
  position: (isize, isize),
  velocity: (isize, isize),
}

pub struct Day14 {}

impl Solver for Day14 {
  type Input = Vec<Robot>;

  type Output1 = String;

  type Output2 = String;

  fn parse(input: &str, _path: &str) -> Self::Input {
    let regex = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    regex
      .captures_iter(input)
      .map(|captures| Robot {
        position: (
          captures.name("px").unwrap().as_str().parse().unwrap(),
          captures.name("py").unwrap().as_str().parse().unwrap(),
        ),
        velocity: (
          captures.name("vx").unwrap().as_str().parse().unwrap(),
          captures.name("vy").unwrap().as_str().parse().unwrap(),
        ),
      })
      .collect()
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    None
  }

  fn part_1(robots: &Self::Input) -> Self::Output1 {
    robots
      .iter()
      .filter_map(|robot| Self::quadrant(Self::simulate(robot, 100).position))
      .counts()
      .values()
      .product::<usize>()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2(robots: &Self::Input) -> Self::Output2 {
    // Significantly faster to clone and run multiple steps in parallell than
    // mutating sequentially (and simulate 1 step at a time).
    // Even faster to do it immutably and in parallell like now
    (1..10000).into_par_iter().for_each(|i| {
      Self::make_image(
        &robots
          .iter()
          .map(|robot| Self::simulate(robot, i))
          .collect(),
      )
      .save(format!("temp/{}.png", i))
      .unwrap()
    });
    "Look at image number 6876".to_string()
  }
}

impl Day14 {
  const WIDTH: isize = 101;
  const HEIGHT: isize = 103;

  fn simulate(Robot { position, velocity }: &Robot, time: isize) -> Robot {
    Robot {
      position: (
        (position.0 + velocity.0 * time).rem_euclid(Self::WIDTH),
        (position.1 + velocity.1 * time).rem_euclid(Self::HEIGHT),
      ),
      velocity: *velocity,
    }
  }

  fn quadrant((x, y): (isize, isize)) -> Option<usize> {
    let (half_w, half_h) = (Self::WIDTH / 2, Self::HEIGHT / 2);
    if x == half_w || y == half_h {
      return None;
    }

    match (x < Self::WIDTH / 2, y < Self::HEIGHT / 2) {
      (true, true) => Some(0),
      (true, false) => Some(1),
      (false, true) => Some(2),
      (false, false) => Some(3),
    }
  }

  fn make_image(robots: &Vec<Robot>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let positions: HashSet<_> = robots.iter().map(|robot| robot.position).collect();
    let mut image_buf = ImageBuffer::new(Self::WIDTH as u32, Self::HEIGHT as u32);

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
      *pixel = if positions.contains(&(x as isize, y as isize)) {
        Rgb([255, 255, 255])
      } else {
        Rgb([0, 0, 0])
      };
    }
    image_buf
  }
}
