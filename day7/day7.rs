extern crate regex;
use regex::Regex;

static INPUT_FILE: &str = "day7/input.txt";

fn main() {
  let input_all = std::fs::read_to_string(INPUT_FILE).unwrap();

  {
    let mut desired = std::collections::HashSet::<&str>::new();
    desired.insert("shiny gold");
    let mut found = std::collections::HashSet::<&str>::new();

    loop {
      let mut next_desired = std::collections::HashSet::<&str>::new();

      let desired_re_prefix = "([a-z ]+) bags contain [^.]*[0-9]+ ";
      let desired_re_suffix = " bag";
      for desired_name in &desired {
        let mut re_str: String = desired_re_prefix.to_string();
        re_str.push_str(desired_name);
        re_str.push_str(desired_re_suffix);
        let re = Regex::new(&re_str).unwrap();

        for caps in re.captures_iter(&input_all) {
          let contains = caps.get(1).unwrap().as_str();
          if !found.contains(contains) {
            next_desired.insert(contains);
            found.insert(contains);
          }
        }
      }
      if next_desired.is_empty() {
        break;
      }
      desired = next_desired;
    }
    println!("Part 1 {}", found.len());
  }
  {
    let mut desired = std::collections::HashSet::<(u32, &str)>::new();
    desired.insert((1, "shiny gold"));

    let mut total_count = 0;

    loop {
      let mut next_desired = std::collections::HashSet::<(u32, &str)>::new();

      for (mult, desired_colour) in &desired {
        let re_suffix = concat!(
          r" bags contain ([0-9]+) ([a-z ]+) bags?",
          r"(?:, ([0-9]+) ([a-z ]+) bags?)?",
          r"(?:, ([0-9]+) ([a-z ]+) bags?)?",
          r"(?:, ([0-9]+) ([a-z ]+) bags?)?",
          r"(?:, ([0-9]+) ([a-z ]+) bags?)?",
          r"(?:, ([0-9]+) ([a-z ]+) bags?)?."
        );
        let mut re_str: String = desired_colour.to_string();
        re_str.push_str(re_suffix);
        let re = Regex::new(&re_str).unwrap();
        let opt_caps = re.captures(&input_all);
        if opt_caps.is_none() {
          continue;
        }
        let caps = opt_caps.unwrap();

        let v : Vec<&str> = caps.iter().skip(1).filter_map(|x| x.and_then(|y| Some(y.as_str()))).collect();
        for i in (0..v.len()).step_by(2) {
          let count = v[i].parse::<u32>().unwrap();
          let name = v[i+1];
          total_count += count * mult;
          next_desired.insert((count * mult, name));
        }
      }

      if next_desired.is_empty() {
        break;
      }
      desired = next_desired;
    }
    println!("Part 2 {}", total_count);
  }
}
