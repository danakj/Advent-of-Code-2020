static INPUT_FILE: &str = "day1/input.txt";

fn main() -> std::io::Result<()> {
  let in_str = std::fs::read_to_string(INPUT_FILE)?;
  let mut numbers = Vec::<i32>::new();
  for s in in_str.split_whitespace() {
    let i = s.parse::<i32>().unwrap();
    numbers.push(i);
  }

  for i in 0..numbers.len() {
    for j in (i + 1)..numbers.len() {
      let (a, b) = (numbers[i], numbers[j]);
      if a + b == 2020 {
        println!("Part 1 {} * {} = {}", a, b, a * b);
      }
    }
  }

  for i in 0..numbers.len() {
    for j in (i + 1)..numbers.len() {
      for k in (j + 1)..numbers.len() {
        let (a, b, c) = (numbers[i], numbers[j], numbers[k]);
        if a + b + c == 2020 {
          println!("Part 2 {} * {} * {} = {}", a, b, c, a * b * c);
        }
      }
    }
  }

  Ok(())
}
