// #[macro_use]
extern crate anyhow;
//extern crate regex;
//use regex::Regex;

fn read_input() -> anyhow::Result<String> {
  //Ok(std::fs::read_to_string("day15/input.txt")?)
  Ok("0,20,7,16,1,18,15".to_owned())
}

enum SpokenTurns {
  Once(/*turn=*/u64),
  Many(/*last_turn=*/u64, /*before_turn=*/u64),
}

struct Part1 {
  history: std::collections::BTreeMap</*spoken_number=*/u64, SpokenTurns>,
  last_spoken: u64,
  current_turn: u64,
}
impl Part1 {
  fn new() -> Part1 {
    Part1 {
      history: std::collections::BTreeMap::new(),
      last_spoken: 0,
      current_turn: 1,
    }
  }

  fn speak(&mut self, spoken_number: u64) {
    let spoken_turns = if self.history.contains_key(&spoken_number) {
      match self.history[&spoken_number] {
        SpokenTurns::Once(last_turn) => SpokenTurns::Many(self.current_turn, last_turn),
        SpokenTurns::Many(last_turn, _) => SpokenTurns::Many(self.current_turn, last_turn),
      }
    } else {
      SpokenTurns::Once(self.current_turn)
    };
    self.history.insert(spoken_number, spoken_turns);
    self.last_spoken = spoken_number;
    self.current_turn += 1;
  }
}

fn run(part: u32, input_all: &str, num_turns_to_run: usize) -> anyhow::Result<()> {
  let split_input = input_all.split_terminator(",");
  let nums = split_input.map(|s: &str| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
  
  let mut state = Part1::new();
  // Bootstrap from `nums`.
  for spoken_num in &nums {
    state.speak(*spoken_num);
  }
  for _ in 0..(num_turns_to_run-nums.len()) {
    let last_spoken = &state.history[&state.last_spoken];
    let next_spoken_number = match last_spoken {
      SpokenTurns::Once(_) => 0,
      SpokenTurns::Many(a, b) => a - b,
    };
    state.speak(next_spoken_number);
  }
  println!("Part {} {}", part, state.last_spoken);
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = read_input()?;
  run(1, &input_all, 2020)?;
  run(2, &input_all, 30000000)?;
  Ok(())
}