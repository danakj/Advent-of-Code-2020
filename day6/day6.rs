static INPUT_FILE: &str = "day6/input.txt";

fn main() {
  let input_all = std::fs::read_to_string(INPUT_FILE).unwrap();

  {
    let mut presence = [false; 26];
    let mut count = 0;
    for s in input_all.split_terminator("\n") {
      if s == "" {
        count += presence.iter().filter(|&&b| b).count();
        presence = [false; 26];
        continue;
      }
      for c in s.chars() {
        presence[c as usize - 'a' as usize] = true;
      }
    }
    count += presence.iter().filter(|&&b| b).count();
    println!("Part 1 {}", count);
  }

  {
    let mut presence = [0u32; 26];
    let mut count = 0;
    let mut group_size = 0;
    for s in input_all.split_terminator("\n") {
      if s == "" {
        count += presence.iter().filter(|&&x| x == group_size).count();
        presence = [0; 26];
        group_size = 0;
        continue;
      }
      group_size += 1;  // Each row between empty lines is a new person in the group.
      for c in s.chars() {
        presence[c as usize - 'a' as usize] += 1;
      }
    }
    count += presence.iter().filter(|&&x| x == group_size).count();
    println!("Part 2 {}", count);
  }
}
