// #[macro_use]
extern crate anyhow;
extern crate regex;

use regex::Regex;

fn read_input() -> anyhow::Result<String> {
  Ok(std::fs::read_to_string("day14/input.txt")?)
  //Ok("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1".to_owned())
}

struct Mask36 {
  mask: [Option<bool>; 36],
}
impl Mask36 {
  fn new() -> Mask36 {
    Mask36 { mask: [None; 36] }
  }

  fn from_str(s: &str) -> Mask36 {
    assert_eq!(s.len(), 36);
    Mask36 {
      mask: {
        let mut mask = [None; 36];
        for (i, c) in s.chars().rev().enumerate() {
          mask[i] = match c {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
          };
        }
        mask
      },
    }
  }
  fn masked_value(&self, mut value: u36) -> u36 {
    for i in 0..36 {
      match self.mask[i] {
        Some(bitvalue) => {
          value.set_bit(i, bitvalue);
        }
        None => (),
      }
    }
    value
  }

  fn masked_addrs(&self, mut addr: u36) -> Vec<u36> {
    for (i, m) in self.mask.iter().enumerate() {
      if let Some(maskval) = m {
        if *maskval {
          addr.bits |= 1 << i;
        }
      }
    }

    let x_indices: Vec<usize> = self
      .mask
      .iter()
      .enumerate()
      .filter(|(_i, x)| x.is_none())
      .map(|(i, _none)| i)
      .collect();
    let addr_drop_xs_mask = {
      let mut inverse = 0_u64;
      for idx in &x_indices {
        inverse |= 1 << *idx;
      }
      !inverse
    };
    // Base address, without any of the mask permutations.
    addr.bits &= addr_drop_xs_mask;
    // Find all permutations of the flipped bits indicated by `x_indices`.
    fn permute(addrs: &mut Vec<u36>, mut x_indices: Vec<usize>, addr_bits: u64) {
      if let Some(idx) = x_indices.pop() {
        permute(addrs, x_indices.clone(), addr_bits);
        let flipped_addr = addr_bits | 1 << idx;
        addrs.push(u36::from_u64(flipped_addr));
        permute(addrs, x_indices, flipped_addr);
      }
    }
    let mut addrs = Vec::<u36>::new();
    addrs.push(addr);
    permute(&mut addrs, x_indices, addr.bits);
    addrs
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
struct u36 {
  bits: u64,
}
impl u36 {
  fn from_u64(input: u64) -> u36 {
    u36 { bits: input }
  }

  fn set_bit(&mut self, bitindex: usize, bitvalue: bool) {
    assert!(bitindex < 36);
    if bitvalue {
      self.bits |= 1 << bitindex;
    } else {
      self.bits &= !(1 << bitindex);
    }
  }
}

struct Array36 {
  array: Vec<(u36, u36)>,
}
impl Array36 {
  fn new() -> Array36 {
    Array36 { array: vec![] }
  }

  fn set(&mut self, addr: u36, value: u36) {
    for i in 0..self.array.len() {
      if self.array[i].0.bits == addr.bits {
        self.array[i].1 = value;
        return;
      } else if self.array[i].0.bits > addr.bits {
        self.array.insert(i, (addr, value));
        return;
      }
    }
    self.array.push((addr, value));
  }
  fn set_group(&mut self, addrs: &Vec<u36>, value: u36) {
    for addr in addrs {
      self.set(*addr, value);
    }
  }

  fn sum(&self) -> u64 {
    self.array.iter().map(|(_i, val)| val.bits).sum()
  }
}

enum Instruction {
  UpdateMask(Mask36),
  SetMem((u36, u36)),
}

fn main() -> anyhow::Result<()> {
  let in_str = read_input()?;
  p1(&in_str)?;
  p2(&in_str)?;
  Ok(())
}

fn parse_instructions(lines: &Vec<&str>) -> anyhow::Result<Vec<Instruction>> {
  let mut instructions = Vec::<Instruction>::new();

  let mask_re = Regex::new(r"^mask = ([X01]{36})$").unwrap();
  let memset_re = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
  for line in lines {
    if let Some(mask_cap) = mask_re.captures(line) {
      let mask = Mask36::from_str(&mask_cap[1]);
      instructions.push(Instruction::UpdateMask(mask));
    } else if let Some(memset_cap) = memset_re.captures(line) {
      let addr = u36::from_u64(memset_cap[1].parse()?);
      let val = u36::from_u64(memset_cap[2].parse()?);
      instructions.push(Instruction::SetMem((addr, val)));
    }
  }
  Ok(instructions)
}

fn p1(in_str: &String) -> anyhow::Result<()> {
  let lines: Vec<&str> = in_str.split_terminator("\n").collect();
  let instructions = parse_instructions(&lines)?;

  let mut cur_mask = Mask36::new();
  let mut memory = Array36::new();
  for inst in instructions {
    match inst {
      Instruction::UpdateMask(mask) => cur_mask = mask,
      Instruction::SetMem((addr, value)) => memory.set(addr, cur_mask.masked_value(value)),
    }
  }

  println!("Part 1 {}", memory.sum());
  Ok(())
}

fn p2(in_str: &String) -> anyhow::Result<()> {
  let lines: Vec<&str> = in_str.split_terminator("\n").collect();
  let instructions = parse_instructions(&lines)?;

  let mut cur_mask = Mask36::new();
  let mut memory = Array36::new();
  for inst in instructions {
    match inst {
      Instruction::UpdateMask(mask) => cur_mask = mask,
      Instruction::SetMem((addr, value)) => memory.set_group(&cur_mask.masked_addrs(addr), value),
    }
  }

  println!("Part 2 {}", memory.sum());
  Ok(())
}

impl std::fmt::Display for Mask36 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut s = "".to_owned();
    for bit in self.mask.iter().rev() {
      s += match bit {
        Some(b) => match b {
          true => "1",
          false => "0",
        },
        None => "X",
      }
    }
    write!(f, "{}", s)?;
    Ok(())
  }
}
impl std::fmt::Display for u36 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.bits)?;
    Ok(())
  }
}
impl std::fmt::Display for Array36 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut s = "".to_owned();
    for (i, val) in self.array.iter() {
      s += &format!("[{}, {}] ", i, val);
    }
    write!(f, "{}", s)?;
    Ok(())
  }
}
