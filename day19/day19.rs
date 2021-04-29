// #[macro_use]
extern crate anyhow;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

struct CombineOptions {
  combined_options: Vec<CombineRules>,
}

struct CombineRules {
  combined_rules: Vec<u64>,
}

enum Rule {
  Combines(CombineOptions),
  Terminates(Vec<char>),
}

struct Rules {
  rules: HashMap<u64, Rule>,
}

impl Rules {
  fn build(lines: Vec<String>) -> Rules {
    let sub_rule_re = Regex::new(r"([0-9]+): ([0-9 |]+)").unwrap();
    let end_rule_re = Regex::new(r#"([0-9]+): "(.*)""#).unwrap();
    let rules: HashMap<u64, Rule> = lines.iter()
      .filter_map(|line| {
        if let Some(caps) = sub_rule_re.captures(line) {
          let rule_num = caps[1].parse().unwrap();
          // An iterator over all CombineRules.
          let sub_rule_it = caps[2].split_terminator("|").map(|rules| {
            // Grab all the rule numbers on each side of the | separators
            // into a CombineRules.
            CombineRules {
              combined_rules: rules.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            }
          });
          Some((rule_num, Rule::Combines(CombineOptions {
            combined_options: sub_rule_it.collect(),
          })))
        } else if let Some(caps) = end_rule_re.captures(line) {
          let rule_num = caps[1].parse().unwrap();
          let string = &caps[2];
          Some((rule_num, Rule::Terminates(string.chars().collect())))
        } else {
          None
        }
      })
      .collect();
    Rules { rules: rules }
  }

  fn build_pattern_permutations(rule_num: &u64, rules: &Rules) -> Vec<Vec<char>> {
    fn combine(prefixes: Vec<Vec<char>>, suffixes: Vec<Vec<char>>) -> Vec<Vec<char>> {
      let mut result = Vec::new();
      for suffix in &suffixes {
        for prefix in &prefixes {
          result.push(vec![prefix.clone(), suffix.clone()].concat());
        }
      }
      result
    }

    match &rules.rules[rule_num] {
      Rule::Terminates(chars) => {
        vec![chars.clone()]
      },
      Rule::Combines(opts) => {
        let mut rule_combined = Vec::new();
        for option in &opts.combined_options {
          let mut one_option: Vec<Vec<char>> = vec![vec![]];
          for rule_num in &option.combined_rules {
            one_option = combine(one_option, Self::build_pattern_permutations(rule_num, rules));
          }
          rule_combined.append(&mut one_option);
        }
        rule_combined
      }
    }
  }

  fn count_matches(&self, rule_num: u64, mut message_strs: Vec<&str>) -> usize {
    message_strs.sort();
    let messages = message_strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut valid_patterns = Self::build_pattern_permutations(&rule_num, &self);
    valid_patterns.sort();

    let mut count = 0;
    let mut m_i = 0;
    let mut p_i = 0;
    loop {
      if m_i >= messages.len() { break }
      if p_i >= valid_patterns.len() { break }

      let message = &messages[m_i];
      let pattern = &valid_patterns[p_i];
      if message == pattern {
        m_i += 1;
        p_i += 1;
        count += 1;
      } else if message < pattern {
        m_i += 1;
      } else {
        p_i += 1;
      }
    }
    count
  }

  // Counts Rule42+Rule31+ where # of 42 > # of 31.
  fn count_repeating_matches(&self, message_strs: Vec<&str>) -> usize {
    let messages = message_strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

  let mut valid_42 = Self::build_pattern_permutations(&42, &self);
    valid_42.sort();
    let mut valid_31 = Self::build_pattern_permutations(&31, &self);
    valid_31.sort();

    let mut count_valid_messages = 0;
    for message in messages {
      // Some number of 42s and then some fewer number of B31s.
      let mut remain = &message[..];
      let mut count_42 = 0;
      let mut count_31 = 0;
      loop {
        let mut any = false;
        for prefix in &valid_42 {
          if remain.starts_with(prefix) {
            count_42 += 1;
            remain = &remain[prefix.len()..];
            any = true;
            break
          }
        }
        if !any { break }
      }
      loop {
        let mut any = false;
        for prefix in &valid_31 {
          if remain.starts_with(prefix) {
            count_31 += 1;
            remain = &remain[prefix.len()..];
            any = true;
            break
          }
        }
        if !any { break }
      }
      if remain.len() == 0 && count_31 > 0 && count_42 > count_31 {
        count_valid_messages += 1;
      }
    }
    count_valid_messages
  }
}

fn p1(input_all: &str) -> anyhow::Result<usize> {
  let rules = Rules::build(input_all.split_terminator("\n").map(|s| s.to_owned()).collect::<Vec<String>>());
  let messages: Vec<&str> = input_all.split_terminator("\n").filter(|&s| s != "" && s.find(":").is_none()).collect();
  Ok(rules.count_matches(0, messages))
}

fn p2(input_all: &str) -> anyhow::Result<usize> {
  let replacer = |s: &str| -> String {
    if s.starts_with("8: ") {
      "8: 42 | 42 8"
    } else if s.starts_with("11: ") {
      "11: 42 31 | 42 11 31"
    } else {
      s
    }.to_owned()
  };
  let rules = Rules::build(input_all.split_terminator("\n").map(replacer).collect::<Vec<String>>());
  let messages: Vec<&str> = input_all.split_terminator("\n").filter(|&s| s != "" && s.find(":").is_none()).collect();
  Ok(rules.count_repeating_matches(messages))
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day19/input.txt")?;
  println!("Part 1 {}", p1(&input_all)?);
  println!("Part 2 {}", p2(&input_all)?);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const P1_TEST_INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
  const P1_OUTPUT: &str = "2";

  #[test]
  fn test_p1() -> anyhow::Result<()> {
    let lines = P1_TEST_INPUT.split_terminator("\n").map(|s| s.to_owned()).collect();
    let rules = Rules::build(lines);
    assert_eq!(rules.count_matches(0, vec!["ababbb"]), 1);
    assert_eq!(rules.count_matches(0, vec!["abbbab"]), 1);
    assert_eq!(rules.count_matches(0, vec!["bababa"]), 0);
    assert_eq!(rules.count_matches(0, vec!["aaabbb"]), 0);
    assert_eq!(rules.count_matches(0, vec!["aaabbb"]), 0);

    assert_eq!(format!("{}", p1(P1_TEST_INPUT)?), P1_OUTPUT);
    Ok(())
  }

  const P2_TEST_INPUT: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
  const P2_OUTPUT: &str = "12";

  #[test]
  fn test_p2() -> anyhow::Result<()> {
    assert_eq!(format!("{}", p2(P2_TEST_INPUT)?), P2_OUTPUT);
    Ok(())
  }
}
