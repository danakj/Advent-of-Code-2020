// #[macro_use]
extern crate anyhow;
//extern crate itertools;
//use itertools::Itertools;
//extern crate regex;
//use regex::Regex;

fn solve(input_all: String) -> anyhow::Result<()> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = if std::env::args().nth(1).filter(|s| s == "test").is_some() {
    std::fs::read_to_string("day??/test.txt")?
  } else {
    std::fs::read_to_string("day??/input.txt")?
  };
  solve(input_all)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
  }
}