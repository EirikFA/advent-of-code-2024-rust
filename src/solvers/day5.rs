use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;

use super::Solver;

type Dependencies = HashMap<u32, Vec<u32>>;
type Prints = Vec<Vec<u32>>;

pub struct Day5 {}

impl Solver for Day5 {
  type Input = (Dependencies, Prints);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut deps: Dependencies = HashMap::new();
        let mut prints: Prints = vec![];

        let mut parsing_deps = true;
        for line in lines.flatten() {
          if line.is_empty() {
            parsing_deps = false;
            continue;
          }

          if parsing_deps {
            Self::parse_dep(line, &mut deps);
          } else {
            Self::parse_print(line, &mut prints);
          }
        }

        (deps, prints)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(143.to_string())
  }

  fn part_1((deps, prints): &Self::Input) -> Self::Output1 {
    let valid = Self::find_valid_prints(prints, deps);
    valid
      .iter()
      .map(|i| prints[*i][prints[*i].len() / 2])
      .sum::<u32>()
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(123.to_string())
  }

  fn part_2((deps, prints): &Self::Input) -> Self::Output2 {
    let valid = Self::find_valid_prints(prints, deps);
    prints
      .iter()
      .enumerate()
      .filter(|(i, _)| !valid.contains(i))
      .map(|(_, page)| {
        let sorted = Self::top_sort(&deps, page);
        sorted[sorted.len() / 2]
      })
      .sum::<u32>()
      .to_string()
  }
}

impl Day5 {
  fn parse_dep(input: String, deps: &mut Dependencies) {
    let parts: Vec<&str> = input.split("|").collect();
    let dependency: u32 = parts[0].parse().unwrap();
    let dependant: u32 = parts[1].parse().unwrap();
    deps.entry(dependency).or_insert(vec![]).push(dependant);
  }

  fn parse_print(input: String, prints: &mut Prints) {
    let pages: Vec<u32> = input.split(",").map(|p| p.parse().unwrap()).collect();
    prints.push(pages);
  }

  fn find_valid_prints(prints: &Prints, deps: &Dependencies) -> HashSet<usize> {
    let mut valid: HashSet<usize> = HashSet::new();
    'pages: for (i, pages) in prints.iter().enumerate() {
      for (p_i, page) in pages.iter().rev().enumerate() {
        for other in pages.iter().rev().skip(p_i + 1) {
          if deps.contains_key(page) && deps[page].contains(other) {
            continue 'pages;
          }
        }
      }

      valid.insert(i);
    }
    valid
  }

  fn top_sort(deps: &Dependencies, pages: &Vec<u32>) -> Vec<u32> {
    let mut order: Vec<u32> = vec![];

    let mut visited: HashSet<u32> = HashSet::new();
    for page in pages {
      Self::dfs_visit(pages, deps, *page, &mut visited, &mut order);
    }

    order.reverse();
    order
  }

  fn dfs_visit(
    pages: &Vec<u32>,
    deps: &Dependencies,
    page: u32,
    visited: &mut HashSet<u32>,
    order: &mut Vec<u32>,
  ) {
    if !pages.contains(&page) || visited.contains(&page) {
      return;
    }

    visited.insert(page);
    for dep in deps.get(&page).unwrap_or(&vec![]) {
      Self::dfs_visit(pages, deps, *dep, visited, order);
    }
    order.push(page);
  }
}
