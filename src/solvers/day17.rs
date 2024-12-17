use std::sync::Mutex;

use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

use crate::utils::read_lines;

use super::Solver;

#[derive(Debug, Clone)]
pub struct Computer {
  pc: u32,
  a: u32,
  b: u32,
  c: u32,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
  Adv,
  Bxl,
  Bst,
  Jnz,
  Bxc,
  Out,
  Bdv,
  Cdv,
}

type Instruction = (Opcode, u32);

pub struct Day17 {}

impl Solver for Day17 {
  type Input = (Computer, Vec<Instruction>, String);

  type Output1 = String;

  type Output2 = String;

  fn parse(_input: &str, path: &str) -> Self::Input {
    match read_lines(path) {
      Ok(lines) => {
        let lines: Vec<String> = lines.flatten().collect();
        let a = Self::parse_line_value(lines[0].as_str());
        let b = Self::parse_line_value(lines[1].as_str());
        let c = Self::parse_line_value(lines[2].as_str());

        let program_reg = Regex::new(r"(\d),(\d)").unwrap();
        let instructions = program_reg
          .captures_iter(lines[4].as_str())
          .map(|captures| {
            let opcode = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let operand = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            (Self::get_opcode(opcode), operand)
          })
          .collect();

        (
          Computer { pc: 0, a, b, c },
          instructions,
          lines[4].split(": ").collect::<Vec<&str>>()[1].to_string(),
        )
      }
      Err(error) => panic!("Error reading lines of input file: {}", error),
    }
  }

  fn part_1_test_output() -> Option<Self::Output1> {
    Some("4,6,3,5,6,3,5,2,1,0".to_string())
  }

  fn part_1((computer, instructions, _program): &Self::Input) -> Self::Output1 {
    let mut computer = computer.clone();
    Self::run_program(&mut computer, instructions)
  }

  fn part_2_test_output() -> Option<Self::Output2> {
    Some(117440.to_string())
  }

  // TODO: ..Solve it
  fn part_2((computer, instructions, program): &Self::Input) -> Self::Output2 {
    // It was worth a try ¯\_(ツ)_/¯
    // (Takes 90 seconds, and 1 billion is probably not even close)
    let lowest_a = Mutex::new(u32::MAX);
    (0..1_000_000_000).par_bridge().for_each(|a| {
      let mut computer = computer.clone();
      computer.a = a;
      computer.b = 0;
      computer.c = 0;
      computer.pc = 0;
      if Self::run_program(&mut computer, instructions) == program.to_string() {
        let mut lowest_a = lowest_a.lock().unwrap();
        if a < *lowest_a {
          *lowest_a = a;
        }
      }
    });

    lowest_a.into_inner().unwrap().to_string()
  }
}

impl Day17 {
  fn parse_line_value(line: &str) -> u32 {
    line.split(": ").collect::<Vec<&str>>()[1].parse().unwrap()
  }

  fn get_opcode(opcode: u32) -> Opcode {
    match opcode {
      0 => Opcode::Adv,
      1 => Opcode::Bxl,
      2 => Opcode::Bst,
      3 => Opcode::Jnz,
      4 => Opcode::Bxc,
      5 => Opcode::Out,
      6 => Opcode::Bdv,
      7 => Opcode::Cdv,
      _ => panic!(),
    }
  }

  fn run_program(computer: &mut Computer, instructions: &Vec<Instruction>) -> String {
    let mut output: Vec<String> = Vec::new();
    while computer.pc / 2 < instructions.len() as u32 {
      let instruction = &instructions[computer.pc as usize / 2];
      if let Some(out) = Self::do_instruction(computer, instruction) {
        output.push(out.to_string());
      }
    }
    output.join(",")
  }

  fn do_instruction(computer: &mut Computer, (opcode, operand): &Instruction) -> Option<u32> {
    let mut out: Option<u32> = None;

    match opcode {
      Opcode::Adv => {
        computer.a = Self::div(*operand, computer);
      }
      Opcode::Bxl => {
        computer.b = computer.b ^ operand;
      }
      Opcode::Bst => computer.b = Self::get_combo_value(*operand, computer) % 8,
      Opcode::Jnz => {
        if computer.a != 0 {
          computer.pc = *operand;
          return None;
        }
      }
      Opcode::Bxc => {
        computer.b = computer.b ^ computer.c;
      }
      Opcode::Out => {
        out = Some(Self::get_combo_value(*operand, computer) % 8);
      }
      Opcode::Bdv => {
        computer.b = Self::div(*operand, computer);
      }
      Opcode::Cdv => {
        computer.c = Self::div(*operand, computer);
      }
    }

    computer.pc += 2;
    out
  }

  fn get_combo_value(operand: u32, computer: &Computer) -> u32 {
    match operand {
      0..=3 => operand,
      4 => computer.a,
      5 => computer.b,
      6 => computer.c,
      _ => panic!(),
    }
  }

  fn div(operand: u32, computer: &Computer) -> u32 {
    let denom = 2_u32.pow(Self::get_combo_value(operand, computer));
    computer.a / denom
  }
}
