static INPUT_FILE: &str = "day9/input.txt";

fn find_sum_of_2(list: &Vec<&u64>, search: &u64) -> bool {
  for i in 0..list.len() {
    for j in i + 1..list.len() {
      if list[i] + list[j] == *search {
        return true;
      }
    }
  }
  return false;
}

fn main() {
  let input_all: String = std::fs::read_to_string(INPUT_FILE).unwrap();
  let nums_all: Vec<u64> = input_all
    .split_terminator("\n")
    .map(|x: &str| x.parse().unwrap())
    .collect();

  let mut preamble: Vec<&u64> = nums_all.iter().take(25).collect();
  let invalid = {
    let mut it = nums_all.iter().skip(25);
    loop {
      let n = it.next().unwrap();
      if !find_sum_of_2(&preamble, n) {
        break n;
      }
      preamble.push(n);
      preamble.remove(0);
      assert!(preamble.len() == 25);
    }
  };
  println!("Part 1 {}", invalid);

  'p2_outer: for begin in 0..nums_all.len() - 2 {
    for end in begin+2..nums_all.len() {
      assert!(begin < end);
      let sum = nums_all[begin..end].iter().fold(0, |sum, i| sum + i);
      if sum == *invalid {
        let min = nums_all[begin..end].iter().min().unwrap();
        let max = nums_all[begin..end].iter().max().unwrap();
        println!("Part 2 {}", min + max);
        break 'p2_outer;
      } else if sum > *invalid {
        break;  // Go to the next `begin`, the sum is too large already.
      }
    }
  }

}
