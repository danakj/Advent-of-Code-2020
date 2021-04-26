// #[macro_use]
extern crate anyhow;
//extern crate regex;
//use regex::Regex;

fn read_input() -> anyhow::Result<String> {
  Ok(std::fs::read_to_string("day15/input.txt")?)
}


fn p1(input_all: &str) -> anyhow::Result<()> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(())
}

fn p2(input_all: &str) -> anyhow::Result<()> {
  let _lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = read_input()?;
  p1(&input_all)?;
  p2(&input_all)?;
  Ok(())
}