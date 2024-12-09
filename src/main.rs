use solvers::Solver;
use std::env;

mod solvers;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = match args.get(1) {
    Some(day) => day,
    None => {
      panic!("Day to run is required");
    }
  };

  run(day);
}

fn run(day: &str) {
  let path = format!("input/day{}", day);
  match day {
    "1" => solvers::day1::Day1::run(&path),
    "2" => solvers::day2::Day2::run(&path),
    "3" => solvers::day3::Day3::run(&path),
    "4" => solvers::day4::Day4::run(&path),
    "5" => solvers::day5::Day5::run(&path),
    "6" => solvers::day6::Day6::run(&path),
    "7" => solvers::day7::Day7::run(&path),
    "8" => solvers::day8::Day8::run(&path),
    "9" => solvers::day9::Day9::run(&path),
    _ => panic!("Day not valid or not implemented"),
  }
}
