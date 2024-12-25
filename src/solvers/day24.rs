use std::collections::HashMap;

use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;

use super::Solver;

type Wire = Option<bool>;
type Wires = HashMap<String, Wire>;
type Gate = (String, (String, String), String);

pub struct Day24 {}

impl Solver for Day24 {
  type Input = (Wires, Vec<Gate>);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let mut parsing_wires = true;
        let wire_regex = Regex::new(r"(\w+): (\d)").unwrap();
        let gate_regex = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();

        let mut wires = Wires::new();
        let mut gates = Vec::new();

        for line in lines.flatten() {
          if line == "" {
            parsing_wires = false;
            continue;
          }

          if parsing_wires {
            let captures = wire_regex.captures(&line).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let value_str = captures.get(2).unwrap().as_str();
            let value = match value_str {
              "0" => Some(false),
              "1" => Some(true),
              _ => None,
            };
            wires.insert(name, value);
          } else {
            let captures = gate_regex.captures(&line).unwrap();
            let input1 = captures.get(1).unwrap().as_str().to_string();
            let gate = captures.get(2).unwrap().as_str().to_string();
            let input2 = captures.get(3).unwrap().as_str().to_string();
            let output = captures.get(4).unwrap().as_str().to_string();
            wires.insert(output.clone(), None);
            gates.push((gate, (input1, input2), output));
          }
        }
        (wires, gates)
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some(2024.to_string())
  }

  fn part_1((wires, gates): &Self::Input) -> Self::Output1 {
    let mut wires = wires.clone();
    while wires
      .iter()
      .any(|(name, value)| name.starts_with("z") && value.is_none())
    {
      Self::do_gates(&mut wires, gates);
    }

    wires
      .iter()
      .filter(|(name, _)| name.starts_with("z"))
      .sorted_by_key(|(name, _)| *name)
      .rev()
      .map(|(_, value)| value.unwrap() as u64)
      .fold(0, |acc, bit| (acc << 1) + bit)
      .to_string()
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    None
  }

  fn part_2((_wires, gates): &Self::Input) -> Self::Output2 {
    let mut sus = Vec::new();
    for (gate, (a_name, b_name), out_name) in gates {
      if out_name != "z45" && out_name.starts_with("z") && gate != "XOR" {
        sus.push((
          gate.clone(),
          (a_name.clone(), b_name.clone()),
          out_name.clone(),
        ));
      }

      if !out_name.starts_with("z")
        && !a_name.starts_with("x")
        && !a_name.starts_with("y")
        && !b_name.starts_with("x")
        && !b_name.starts_with("y")
        && gate == "XOR"
      {
        sus.push((
          gate.clone(),
          (a_name.clone(), b_name.clone()),
          out_name.clone(),
        ));
      }
    }
    dbg!(&sus, sus.len());
    panic!();
  }
}

impl Day24 {
  fn do_gates(wires: &mut Wires, gates: &Vec<Gate>) {
    for (gate, (a_name, b_name), out_name) in gates {
      let a = wires.get(a_name).unwrap();
      let b = wires.get(b_name).unwrap();
      if a.is_none() || b.is_none() {
        continue;
      }
      wires.insert(
        out_name.clone(),
        Some(Self::do_gate(gate, a.unwrap(), b.unwrap())),
      );
    }
  }

  fn do_gate(gate: &String, a: bool, b: bool) -> bool {
    match gate.as_str() {
      "AND" => a && b,
      "OR" => a || b,
      "XOR" => a ^ b,
      _ => panic!(),
    }
  }
}
