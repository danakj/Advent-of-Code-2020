#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

static INPUT_FILE: &str = "day8/input.txt";

#[derive(Clone, Copy)]
enum Operation {
  Nop,
  Acc,
  Jmp,
}

struct Instruction {
  op: Operation,
  arg: i64,
}

fn read_operation(s: &str) -> Operation {
  if s == "nop" {
    Operation::Nop
  } else if s == "acc" {
    Operation::Acc
  } else if s == "jmp" {
    Operation::Jmp
  } else {
    panic!("Invalid operation {}", s);
  }
}

fn read_instruction(line: &str) -> Instruction {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(nop|acc|jmp) ([+-][0-9]+)").unwrap();
  };
  let captures = RE.captures(line).unwrap();
  let op_match = captures.get(1).unwrap();
  let arg_match = captures.get(2).unwrap();
  Instruction {
    op: read_operation(op_match.as_str()),
    arg: arg_match.as_str().parse().unwrap(),
  }
}

// Add a signed i64 to an unsigned usize.
fn add_signed(u: usize, i: i64) -> usize {
  use std::convert::TryFrom;
  if i >= 0 {
    u + usize::try_from(i.unsigned_abs()).unwrap()
  } else {
    u - usize::try_from(i.unsigned_abs()).unwrap()
  }
}

fn main() {
  let input_all = std::fs::read_to_string(INPUT_FILE).unwrap();
  p1(&input_all);
  p2(&input_all);
}

fn p1(input_all: &str) {
  // Pairs of:
  // bool: Was this line executed yet
  // Instruction: instruction to run
  let mut program: Vec<(bool, Instruction)> = input_all
    .split_terminator("\n")
    .map(|s| (false, read_instruction(s)))
    .collect();

  // Global accumulator for the program to write to.
  let mut accumulator: i64 = 0;
  // Line of the `program` that is running.
  let mut pc: usize = 0;

  loop {
    // Abort when trying to run an instruction that was already run.
    if program[pc].0 {
      break;
    }
    program[pc].0 = true;

    let instruction = &program[pc].1;
    match instruction.op {
      Operation::Nop => {
        pc += 1;
      }
      Operation::Acc => {
        accumulator += instruction.arg;
        pc += 1;
      }
      Operation::Jmp => {
        pc = add_signed(pc, instruction.arg);
      }
    }
  }

  println!("Part 1 {}", accumulator);
}

fn p2(input_all: &str) {
  struct ProgramLine {
    // Was this line executed yet in the current execution.
    visited: bool,
    // Was this instruction's Operation flipped yet in the current
    // or any previous execution.
    op_flipped: bool,
    // Instruction to run.
    instruction: Instruction,
  }
  let mut program: Vec<ProgramLine> = input_all
    .split_terminator("\n")
    .map(|line| ProgramLine {
      visited: false,
      op_flipped: false,
      instruction: read_instruction(line),
    })
    .collect();

  // Exceute the program over and over, trying to flip a single instruction
  // each time. Once successful termination happens, the loop will resolve
  // to the accumulator value at the end of that execution.
  let result = 'execute_program: loop {
    // Global accumulator for the program to write to.
    let mut accumulator: i64 = 0;
    // Line of the `program` that is running.
    let mut pc: usize = 0;
    // Becomes true once a nop/jmp has been flipped for this execution.
    let mut flipped_an_op = false;
    // Reset visited for each execution of the program.
    for pline in &mut program {
      pline.visited = false;
    }

    // A single execution of the program, which loops until the program
    // terminates successfully or enters an infinite loop.
    loop {
      // Successful program termination if we reach 1 past the end of the program,
      // so stop trying to execute the program and return the accumulator.
      if pc == program.len() {
        break 'execute_program accumulator;
      }

      let pline = &mut program[pc];

      // Abort when trying to run an instruction that was already run.
      if pline.visited {
        break;
      }
      pline.visited = true;

      // Try flip an instruction if we haven't tried flipping this instruction
      // and we haven't flipped any other instruction on this exectution.
      let op_to_run = if flipped_an_op || pline.op_flipped {
        pline.instruction.op
      } else {
        match pline.instruction.op {
          Operation::Nop => {
            pline.op_flipped = true;
            flipped_an_op = true;
            Operation::Jmp
          }
          Operation::Jmp => {
            pline.op_flipped = true;
            flipped_an_op = true;
            Operation::Nop
          }
          Operation::Acc => Operation::Acc, // No flip.
        }
      };

      match op_to_run {
        Operation::Nop => {
          pc += 1;
        }
        Operation::Acc => {
          accumulator += pline.instruction.arg;
          pc += 1;
        }
        Operation::Jmp => {
          pc = add_signed(pc, pline.instruction.arg);
        }
      }
    }
  };

  println!("Part 2 {}", result);
}
