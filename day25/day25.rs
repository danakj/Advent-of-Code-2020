// The loop size is a secret, but once you know it this
// can produce a public key or an encryption key.
// transform(7, my loop size) => my public key.
// transform(other party public key, my loop size) => encryption key.
fn transform(subject_number: usize, loop_size: usize) -> usize {
  let mut value = 1;
  for _ in 0..loop_size {
    value *= subject_number;
    value %= 20201227;
  }
  value
}

// Hacking the planet.
fn find_loop_size(pubkey: usize) -> usize {
  let mut value = 1;
  let subject_number = 7;
  let mut loop_size = 0;
  loop {
    value *= subject_number;
    value %= 20201227;
    loop_size += 1;
    if value == pubkey { break loop_size }
  }
}

fn solve(input_all: String) {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();

  let pubkey1: usize = lines[0].parse().expect("bad input 1");
  let pubkey2: usize = lines[1].parse().expect("bad input 2");
  let loop_size1 = find_loop_size(pubkey1);
  // let loop2 = find_loop_size(pubkey2);  // Not needed.
  let encryption_key = transform(pubkey2, loop_size1);
  println!("Part 1 {}", encryption_key);
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let input_all = if std::env::args().nth(1).filter(|s| s == "test").is_some() {
    std::fs::read_to_string("day25/test.txt")?
  } else {
    std::fs::read_to_string("day25/input.txt")?
  };
  solve(input_all);
  Ok(())
}
