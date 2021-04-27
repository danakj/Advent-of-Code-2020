//#[macro_use]
extern crate anyhow;
//extern crate regex;
//use regex::Regex;
extern crate bit_set;
use bit_set::BitSet;

const USE_TEST_INPUT: bool = false;
const TEST_INPUT: &str = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

fn read_input() -> anyhow::Result<String> {
  if USE_TEST_INPUT {
    Ok(TEST_INPUT.to_owned())
  } else {
    Ok(std::fs::read_to_string("day16/input.txt")?)
  }
}

struct FieldRange {
  low: u64,
  high: u64,
}

struct FieldDefn {
  name: String,
  ranges: Vec<FieldRange>,
}
impl std::fmt::Display for FieldDefn {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut s = format!(
      "{}: {}-{}",
      self.name, self.ranges[0].low, self.ranges[0].high
    );
    for r in self.ranges.iter().skip(1) {
      s += &format!(" or {}-{}", r.low, r.high);
    }
    write!(f, "{}", s)?;
    Ok(())
  }
}
impl FieldDefn {
  fn from_str(s: &str) -> FieldDefn {
    let mut split = s.split_terminator(": ");
    let name = split.next().unwrap();
    let ranges_all = split.next().unwrap();
    assert_eq!(split.next(), None);
    let ranges = {
      let mut vec = Vec::<FieldRange>::new();
      let ranges_strs = ranges_all.split_terminator(" or ");
      for range_str in ranges_strs {
        let mut low_high = range_str.split_terminator("-");
        let low = low_high
          .next()
          .expect("bad low string")
          .parse()
          .expect("parse low string");
        let high = low_high
          .next()
          .expect("bad high string")
          .parse()
          .expect("parse high string");
        assert_eq!(low_high.next(), None);
        vec.push(FieldRange {
          low: low,
          high: high,
        });
      }
      vec
    };
    FieldDefn {
      name: name.to_owned(),
      ranges: ranges,
    }
  }
}

struct Inputs<'a> {
  fields: Vec<&'a str>,
  your_ticket: &'a str,
  nearby_tickets: Vec<&'a str>,
}
impl<'a> Inputs<'_> {
  fn from_str(input_all: &'a str) -> Inputs<'a> {
    let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
    let mut it = lines.iter();
    let field_lines = {
      let mut v = Vec::<&str>::new();
      while let Some(s) = it.next() {
        if *s == "" {
          break;
        }
        v.push(s);
      }
      v
    };
    assert_eq!(it.next(), Some(&"your ticket:"));
    let your_ticket_line = it.next().unwrap();
    assert_eq!(it.next(), Some(&""));
    assert_eq!(it.next(), Some(&"nearby tickets:"));
    let nearby_ticket_lines = {
      let mut v = Vec::<&str>::new();
      while let Some(s) = it.next() {
        v.push(s);
      }
      v
    };
    Inputs {
      fields: field_lines,
      your_ticket: your_ticket_line,
      nearby_tickets: nearby_ticket_lines,
    }
  }

  fn get_field_defns(&self) -> Vec<FieldDefn> {
    self.fields.iter().map(|s| FieldDefn::from_str(s)).collect()
  }

  fn get_nearby_tickets(&self) -> Vec<Vec<u64>> {
    let mut nearbys = Vec::new();
    for nearby in &self.nearby_tickets {
      let split = nearby.split_terminator(",");
      nearbys.push(split.map(|s| s.parse().unwrap()).collect());
    }
    nearbys
  }

  fn get_your_ticket(&self) -> Vec<u64> {
    self
      .your_ticket
      .split_terminator(",")
      .map(|s| s.parse().unwrap())
      .collect()
  }
}

fn p1(input_all: &str) -> anyhow::Result<()> {
  let inputs = Inputs::from_str(input_all);

  let mut bad_sum = 0;

  let field_defns: Vec<FieldDefn> = inputs.get_field_defns();
  for nearby_str in inputs.nearby_tickets {
    let nearby_field_nums: Vec<u64> = nearby_str
      .split_terminator(",")
      .map(|s| s.parse().unwrap())
      .collect();
    for n in nearby_field_nums {
      let mut found = false;
      'search: for field_defn in &field_defns {
        for range in &field_defn.ranges {
          if n >= range.low && n <= range.high {
            found = true;
            break 'search;
          }
        }
      }
      if !found {
        bad_sum += n;
      }
    }
  }
  println!("Part 1 {}", bad_sum);
  Ok(())
}

fn p2(input_all: &str) -> anyhow::Result<()> {
  let inputs = Inputs::from_str(input_all);
  let field_defns = inputs.get_field_defns();
  let num_fields = field_defns.len();

  let mut possible_fields = std::collections::HashMap::</*FieldDefn.name=*/ String, BitSet>::new();
  // Mark every field as possible for each FieldDefn.
  for field in &field_defns {
    let mut bitset = BitSet::with_capacity(num_fields);
    for i in 0..num_fields {
      bitset.insert(i);
    }
    possible_fields.insert(field.name.clone(), bitset);
  }

  let nearbys: Vec<Vec<u64>> = {
    let all_nearbys = inputs.get_nearby_tickets();
    // Drop invalid tickets from the `inputs`.
    let is_valid_ticket = |ticket: &Vec<u64>| {
      'search: for &n in ticket.iter() {
        for field_defn in &field_defns {
          for range in &field_defn.ranges {
            if n >= range.low && n <= range.high {
              continue 'search;
            }
          }
        }
        return false;
      }
      true
    };
    all_nearbys.into_iter().filter(is_valid_ticket).collect()
  };
  // Walk through the nearby tickets eliminating possible field indices from FieldDefns.
  for n in nearbys {
    for (i, val) in n.into_iter().enumerate() {
      // For the `i`th field on a ticket, see if this value excludes any FieldDefn.
      for fdefn in &field_defns {
        let mut fits_field_defn = false;
        for frange in &fdefn.ranges {
          if val >= frange.low && val <= frange.high {
            fits_field_defn = true;
            break;
          }
        }
        if !fits_field_defn {
          // Remove `i` from the possible field indices for FieldDefn.
          possible_fields.get_mut(&fdefn.name).unwrap().remove(i);
        }
      }
    }
  }

  // For each FieldDefn that has only a single possible field index, remove that
  // field index from the other FieldDefns, and repeat this recursively. We should
  // have a single field index per FieldDefn at the end in order to solve the
  // problem!
  fn reduce(
    possible_fields: &mut std::collections::HashMap<String, BitSet>,
    field_defns: &Vec<FieldDefn>,
    fdefn: &FieldDefn,
  ) {
    assert_eq!(possible_fields[&fdefn.name].len(), 1);
    for other_fdefn in field_defns {
      assert!(possible_fields[&other_fdefn.name].len() >= 1);
      if fdefn.name == other_fdefn.name {
        continue;
      }
      if possible_fields[&other_fdefn.name].len() == 1 {
        continue;
      }

      let remove_bit = possible_fields[&fdefn.name].iter().next().unwrap();
      possible_fields
        .get_mut(&other_fdefn.name)
        .unwrap()
        .remove(remove_bit);
      if possible_fields[&other_fdefn.name].len() == 1 {
        // Recurse if this fdefn now has a unique field index, to remove
        // that field index from the rest.
        reduce(possible_fields, field_defns, other_fdefn);
      }
    }
  }
  for fdefn in &field_defns {
    if possible_fields[&fdefn.name].len() == 1 {
      reduce(&mut possible_fields, &field_defns, fdefn);
    }
  }

  let your_ticket: Vec<u64> = inputs.get_your_ticket();

  let mut depart_prod = 1;
  for fdefn in &field_defns {
    assert_eq!(possible_fields[&fdefn.name].len(), 1);
    if fdefn.name.starts_with("departure") {
      depart_prod *= your_ticket[possible_fields[&fdefn.name].iter().next().unwrap()];
    }
  }
  println!("Part 2 {}", depart_prod);
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = read_input()?;
  p1(&input_all)?;
  p2(&input_all)?;
  Ok(())
}
