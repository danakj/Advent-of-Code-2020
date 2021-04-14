static INPUT_FILE: &str = "day10/input.txt";

fn main() {
  let input_all: String = std::fs::read_to_string(INPUT_FILE).unwrap();
  let mut nums_all : Vec<u32> = input_all.split_terminator("\n").map(|x| x.parse().unwrap()).collect();
  nums_all.sort();

  let mut current_jolts = 0;
  let mut count_1_diffs = 0;
  let mut count_3_diffs = 1;  // From the last adaptor to the device is always 3 jolts.
  for n in &nums_all {
    assert!(*n <= current_jolts + 3);
    match n - current_jolts {
      1 => count_1_diffs += 1,
      3 => count_3_diffs += 1,
      _ => (),
    }
    current_jolts = *n;
  }
  println!("Part 1 {}", count_1_diffs * count_3_diffs);

  struct PathStep {
    paths_to_end : u64,
    jolts : u32,
  }
  impl PathStep {
    fn new(jolts : u32) -> Self {
      Self{paths_to_end: 0, jolts: jolts}
    }
    fn new_with_path(jolts : u32) -> Self {
      Self{paths_to_end: 1, jolts: jolts}
    }
  }
  let mut jolt_set: Vec<PathStep> = nums_all.iter().map(|x| PathStep::new(*x)).collect();
  jolt_set.insert(0, PathStep::new(0));
  jolt_set.push(PathStep::new_with_path(jolt_set.last().unwrap().jolts + 3));

  for i in (0..jolt_set.len() - 1).rev() {
    let current_jolts = jolt_set[i].jolts;
    for j in i+1..=i+3 {
      if j == jolt_set.len() { break; }
      if jolt_set[j].jolts > current_jolts + 3 { break; }

      jolt_set[i].paths_to_end += jolt_set[j].paths_to_end;
    }
  }
  println!("Part 2 {}", jolt_set[0].paths_to_end);
}