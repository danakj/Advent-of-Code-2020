// #[macro_use]
extern crate anyhow;
//extern crate regex;
//use regex::Regex;

fn read_input() -> anyhow::Result<String> {
  Ok(std::fs::read_to_string("day17/input.txt")?)
}

struct DimensionRange {
  min: i64,
  max: i64,
}
impl DimensionRange {
  fn absorb(&mut self, i: i64) {
    self.max = std::cmp::max(self.max, i);
    self.min = std::cmp::min(self.min, i);
  }
  fn set_range(&mut self, i: i64) {
    self.max = i;
    self.min = i;
  }
}

struct Dimension3 {
  empty: bool,
  values: std::collections::HashSet<(i64, i64, i64)>,
  xrange: DimensionRange,
  yrange: DimensionRange,
  zrange: DimensionRange,
}
impl Dimension3 {
  fn new() -> Self {
    Self {
      empty: true,
      values: std::collections::HashSet::new(),
      xrange: DimensionRange { min: 0, max: 0 },
      yrange: DimensionRange { min: 0, max: 0 },
      zrange: DimensionRange { min: 0, max: 0 },
    }
  }

  fn get_active(&self, x: i64, y: i64, z: i64) -> bool {
    self.values.get(&(x, y, z)).is_some()
  }

  fn get_active_neighbour_count(&self, x: i64, y: i64, z: i64) -> usize {
    let mut count = 0;
    for xd in -1..=1 {
      for yd in -1..=1 {
        for zd in -1..=1 {
          if xd != 0 || yd != 0 || zd != 0 {
            count += self.get_active(x+xd, y+yd, z+zd) as usize;
          }
        }
      }
    }
    count
  }

  fn get_active_count(&self) -> usize {
    self.values.len()
  }

  fn set_active(&mut self, x: i64, y: i64, z: i64) -> bool {
    if self.empty {
      self.xrange.set_range(x);
      self.yrange.set_range(y);
      self.zrange.set_range(z);
    } else {
      self.xrange.absorb(x);
      self.yrange.absorb(y);
      self.zrange.absorb(z);
    }
    self.empty = false;
    !self.values.insert((x, y, z))
  }

  fn iterate(&self) -> Self {
    let mut next = Dimension3::new();
    for x in self.xrange.min-1..=self.xrange.max+1 {
      for y in self.yrange.min-1..=self.yrange.max+1 {
        for z in self.zrange.min-1..=self.zrange.max+1 {
          let ncount = self.get_active_neighbour_count(x, y, z);
          let next_is_active = if self.get_active(x, y, z) {
            // If active, stay active if 2 or 3 active neighbours.
            ncount == 2 || ncount == 3
          } else {
            // If inactive, become active if 3 active neighbours.
            ncount == 3
          };
          if next_is_active {
            next.set_active(x, y, z);
          }
        }
      }
    }
    next
  }

  fn from_str(string: &str) -> Self {
    let lines: Vec<&str> = string.split_terminator("\n").collect();
    let mut dimension = Dimension3::new();
    for (y, line) in lines.into_iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          dimension.set_active(x as i64, y as i64, 0);
        }
      }
    }
    dimension
  }
}

struct Dimension4 {
  empty: bool,
  values: std::collections::HashSet<(i64, i64, i64, i64)>,
  xrange: DimensionRange,
  yrange: DimensionRange,
  zrange: DimensionRange,
  wrange: DimensionRange,
}
impl Dimension4 {
  fn new() -> Self {
    Self {
      empty: true,
      values: std::collections::HashSet::new(),
      xrange: DimensionRange { min: 0, max: 0 },
      yrange: DimensionRange { min: 0, max: 0 },
      zrange: DimensionRange { min: 0, max: 0 },
      wrange: DimensionRange { min: 0, max: 0 },
    }
  }

  fn get_active(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
    self.values.get(&(x, y, z, w)).is_some()
  }

  fn get_active_neighbour_count(&self, x: i64, y: i64, z: i64, w:i64) -> usize {
    let mut count = 0;
    for xd in -1..=1 {
      for yd in -1..=1 {
        for zd in -1..=1 {
          for wd in -1..=1 {
            if xd != 0 || yd != 0 || zd != 0 || wd != 0 {
              count += self.get_active(x+xd, y+yd, z+zd, w+wd) as usize;
            }
          }
        }
      }
    }
    count
  }

  fn get_active_count(&self) -> usize {
    self.values.len()
  }

  fn set_active(&mut self, x: i64, y: i64, z: i64, w: i64) -> bool {
    if self.empty {
      self.xrange.set_range(x);
      self.yrange.set_range(y);
      self.zrange.set_range(z);
      self.wrange.set_range(w);
    } else {
      self.xrange.absorb(x);
      self.yrange.absorb(y);
      self.zrange.absorb(z);
      self.wrange.absorb(w);
    }
    self.empty = false;
    !self.values.insert((x, y, z, w))
  }

  fn iterate(&self) -> Self {
    let mut next = Dimension4::new();
    for x in self.xrange.min-1..=self.xrange.max+1 {
      for y in self.yrange.min-1..=self.yrange.max+1 {
        for z in self.zrange.min-1..=self.zrange.max+1 {
          for w in self.wrange.min-1..=self.wrange.max+1 {
            let ncount = self.get_active_neighbour_count(x, y, z, w);
            let next_is_active = if self.get_active(x, y, z, w) {
              // If active, stay active if 2 or 3 active neighbours.
              ncount == 2 || ncount == 3
            } else {
              // If inactive, become active if 3 active neighbours.
              ncount == 3
            };
            if next_is_active {
              next.set_active(x, y, z, w);
            }
          }
        }
      }
    }
    next
  }

  fn from_str(string: &str) -> Self {
    let lines: Vec<&str> = string.split_terminator("\n").collect();
    let mut dimension = Dimension4::new();
    for (y, line) in lines.into_iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          dimension.set_active(x as i64, y as i64, /*z=*/0, /*w=*/0);
        }
      }
    }
    dimension
  }

  fn to_str(&self, z: i64, w: i64) -> String {
    let mut s = String::new();
    for y in self.yrange.min..=self.yrange.max {
      for x in self.xrange.min..=self.xrange.max {
        s += if self.get_active(x, y, z, w) { "#" } else { "." }
      }
      s += "\n";
    }
    s
  }

  #[allow(dead_code)]
  fn print(&self) {
    for w in self.wrange.min..=self.wrange.max {
      for z in self.zrange.min..=self.zrange.max {
        println!("z={}, w={}", z, w);
        println!("{}\n", self.to_str(/*z=*/z, /*w=*/w));
      }
    }
    }
}

fn p1(mut dimension: Dimension3) -> anyhow::Result<()> {
  for _ in 0..6 {
    dimension = dimension.iterate();
  }
  println!("Part 1 {}", dimension.get_active_count());
  Ok(())
}

fn p2(mut dimension: Dimension4) -> anyhow::Result<()> {
  for _ in 0..6 {
    dimension = dimension.iterate();
  }
  println!("Part 2 {}", dimension.get_active_count());
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = read_input()?;
  p1(Dimension3::from_str(&input_all))?;
  p2(Dimension4::from_str(&input_all))?;
  Ok(())
}