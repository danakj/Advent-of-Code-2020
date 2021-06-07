// #[macro_use]
extern crate anyhow;
extern crate itertools;
use itertools::Itertools;
//extern crate regex;
//use regex::Regex;

fn p1(input_all: &str) -> anyhow::Result<u64> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(0)
}

fn p2(input_all: &str) -> anyhow::Result<u64> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(0)
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

  #[test]
  fn test_p1() -> anyhow::Result<()> {
    assert_eq!(p1("")?, 0);
    Ok(())
  }

  #[test]
  fn test_p2() -> anyhow::Result<()> {
    assert_eq!(p2("")?, 0);
    Ok(())
  }
}