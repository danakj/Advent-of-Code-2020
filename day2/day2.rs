extern crate regex;

use regex::Regex;

static INPUT_FILE: &str = "day2/input.txt";

struct Rule<'a> {
  low: usize,
  high: usize,
  letter: char,
  password: &'a str,
}

fn read_file_lines(name: &str) -> Vec<String> {
  let in_str: String = match std::fs::read_to_string(name) {
    Ok(s) => s,
    Err(e) => panic!("{}", e),
  };
  return in_str.split_terminator('\n').map(String::from).collect();
}

fn main() {
  let lines: Vec<String> = read_file_lines(INPUT_FILE);

  let mut rules = Vec::<Rule>::new();
  let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
  for line in &lines {
    let captures = re.captures(&line).unwrap();
    rules.push(Rule {
      low: captures[1].parse().unwrap(),
      high: captures[2].parse().unwrap(),
      letter: captures[3].chars().next().unwrap(),
      password: captures.get(4).unwrap().as_str(),
    });
  }

  let valid_rules1 = rules.iter().fold(0, |valid_rule_count : usize, r: &Rule| {
    let count_rule_letter = |count, c| count + (c == r.letter) as usize;
    let letter_count = r.password.chars().fold(0, count_rule_letter);

    valid_rule_count + (letter_count >= r.low && letter_count <= r.high) as usize
  });
  println!("Part 1 {}", valid_rules1);

  let valid_rules2 = rules.iter().fold(0, |valid_rule_count : usize, r: &Rule| {
    let find_rule_letter_at_n = |n: usize| r.password.chars().nth(n - 1).unwrap() == r.letter;

    let mut count_letters = 0;
    count_letters += find_rule_letter_at_n(r.low) as u8;
    count_letters += find_rule_letter_at_n(r.high) as u8;
    valid_rule_count + (count_letters == 1) as usize
  });
  println!("Part 2 {}", valid_rules2);
}
