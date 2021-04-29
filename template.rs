// #[macro_use]
extern crate anyhow;
//extern crate regex;
//use regex::Regex;

fn p1(input_all: &str) -> anyhow::Result<String> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(String::new())
}

fn p2(input_all: &str) -> anyhow::Result<String> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(String::new())
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day??/input.txt")?;
  println!("Part 1 {}", p1(&input_all)?);
  println!("Part 2 {}", p2(&input_all)?);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = r#""#;
  const P1_OUTPUT: &str = "";
  const P2_OUTPUT: &str = "";

  #[test]
  fn test_p1() -> anyhow::Result<()> {
    assert_eq!(p1(TEST_INPUT), P1_OUTPUT);
  }

  #[test]
  fn test_p2() -> anyhow::Result<()> {
    assert_eq!(p2(TEST_INPUT), P2_OUTPUT);
  }
}