#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

static INPUT_FILE: &str = "day4/input.txt";

fn read_file_lines(name: &str) -> Vec<String> {
  let in_str: String = match std::fs::read_to_string(name) {
    Ok(s) => s,
    Err(e) => panic!("{}", e),
  };
  return in_str.split_terminator('\n').map(String::from).collect();
}

fn is_passport(line: &String) -> bool {
  lazy_static! {
    static ref RE_BYR: Regex = Regex::new(r"\bbyr:([0-9]{4})\b").unwrap();
    static ref RE_IYR: Regex = Regex::new(r"\biyr:([0-9]{4})\b").unwrap();
    static ref RE_EYR: Regex = Regex::new(r"\beyr:([0-9]{4})\b").unwrap();
    static ref RE_HGT: Regex = Regex::new(r"\bhgt:([0-9]+)(cm|in)\b").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"\bhcl:(#[0-9a-f]{6})\b").unwrap();
    static ref RE_ECL: Regex = Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    static ref RE_PID: Regex = Regex::new(r"\bpid:([0-9]{9})\b").unwrap();
    static ref RE_CID: Regex = Regex::new(r"\bcid:([^ ]+)\b").unwrap();
  }

  let byr = RE_BYR
    .captures(line)
    .and_then(|cap| cap.get(1))
    .and_then(|m| m.as_str().parse::<u32>().ok())
    .filter(|v| *v >= 1920 && *v <= 2002);
  let iyr = RE_IYR
    .captures(line)
    .and_then(|cap| cap.get(1))
    .and_then(|m| m.as_str().parse::<u32>().ok())
    .filter(|v| *v >= 2010 && *v <= 2020);
  let eyr = RE_EYR
    .captures(line)
    .and_then(|cap| cap.get(1))
    .and_then(|m| m.as_str().parse::<u32>().ok())
    .filter(|v| *v >= 2020 && *v <= 2030);
  let hcl = RE_HCL.captures(line);
  let ecl = RE_ECL.captures(line);
  let pid = RE_PID.captures(line);
  let hgt = RE_HGT.captures(line).and_then(|m| {
    let low;
    let high;
    if &m[2] == "in" {
      low = 59;
      high = 76;
    } else {
      low = 150;
      high = 193;
    }
    m[1].parse::<u32>().ok().filter(|v| *v >= low && *v <= high)
  });

  byr
    .and(iyr)
    .and(eyr)
    .and(hcl)
    .and(ecl)
    .and(pid)
    .and(hgt)
    .is_some()
}

fn main() {
  let lines: Vec<String> = read_file_lines(INPUT_FILE);
  let mut acc_str = String::new();
  let mut count = 0;

  for s in &lines {
    if s == "" {
      count += is_passport(&acc_str) as usize;
      acc_str.clear();
    } else {
      acc_str.push(' ');
      acc_str.push_str(s);
    }
  }
  count += is_passport(&acc_str) as usize;

  println!("{}", count);
}
