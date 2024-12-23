use std::collections::{HashMap, HashSet};

use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;

use super::Solver;

type Computer = String;

pub struct Day23 {}

impl Solver for Day23 {
  type Input = HashMap<Computer, HashSet<Computer>>;

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut edges = HashMap::new();
        let regex = Regex::new(r"([a-z]+)-([a-z]+)").unwrap();
        for line in lines.flatten() {
          let captures = regex.captures(&line).unwrap();
          let from = captures.get(1).unwrap().as_str().to_string();
          let to = captures.get(2).unwrap().as_str().to_string();
          edges
            .entry(from.clone())
            .or_insert_with(HashSet::new)
            .insert(to.clone());
          edges
            .entry(to.clone())
            .or_insert_with(HashSet::new)
            .insert(from.clone());
        }
        edges
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(7.to_string())
  }

  fn part_1(edges: &Self::Input) -> Self::Output1 {
    let mut sets = HashSet::new();
    for (u, u_n) in edges {
      for v in u_n {
        for (w, w_n) in edges {
          let has_t = u.starts_with("t") || v.starts_with("t") || w.starts_with("t");
          if has_t && w_n.contains(u) && w_n.contains(v) {
            let mut set = vec![u.clone(), v.clone(), w.clone()];
            set.sort();
            sets.insert(set);
          }
        }
      }
    }
    sets.len().to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some("co,de,ka,ta".to_string())
  }

  fn part_2(edges: &Self::Input) -> Self::Output2 {
    let mut cliques = Vec::new();
    Self::bron_kerbosch(
      HashSet::new(),
      &mut edges.keys().map(|u| u.clone()).collect(),
      &mut HashSet::new(),
      edges,
      &mut cliques,
    );
    cliques
      .iter()
      .max_by_key(|clique| clique.len())
      .unwrap()
      .iter()
      .sorted()
      .join(",")
  }
}

impl Day23 {
  // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#With_pivoting
  // ~400ms without pivoting, ~15ms with (latter is even faster than part 1)
  // No idea if putting .clone() everywhere is "the Rust way"
  fn bron_kerbosch(
    r: HashSet<Computer>,
    p: &mut HashSet<Computer>,
    x: &mut HashSet<Computer>,
    edges: &HashMap<Computer, HashSet<Computer>>,
    maximal_cliques: &mut Vec<HashSet<Computer>>,
  ) {
    if p.is_empty() && x.is_empty() {
      maximal_cliques.push(r.clone());
    }

    let pivot = p.iter().next();
    if pivot.is_none() {
      return;
    }

    for v in p.clone().difference(edges.get(pivot.unwrap()).unwrap()) {
      let v_n = edges.get(v).unwrap();
      Self::bron_kerbosch(
        r.union(&HashSet::from([v.clone()]))
          .map(|u| u.clone())
          .collect(),
        &mut p.intersection(v_n).map(|u| u.clone()).collect(),
        &mut x.intersection(v_n).map(|u| u.clone()).collect(),
        edges,
        maximal_cliques,
      );
      p.remove(v);
      x.insert(v.clone());
    }
  }
}
