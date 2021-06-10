// #[macro_use]
extern crate anyhow;
//extern crate itertools;
//use itertools::Itertools;
//extern crate regex;
//use regex::Regex;
use std::collections::HashMap;

// Directions:
//
//  NW /\ NE
//    /  \
//   |    |
// W |    | E
//   |    |
//    \  /
//  SW \/ SE
//
// Coords (x inline):
//
// y = -1   -5  -3  -1   1   3   5
// y =  0     -4  -2   0   2   4   6
// y =  1   -5  -3  -1   1   3   5
//
// So there's no even x coords on odd rows.
// And there's no odd x coords on even rows.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Side {
  White,
  Black,
}

struct Tile {
  face: Side,
}
impl Tile {
  fn new() -> Self {
    Tile { face: Side::White }
  }

  fn flip(&mut self) {
    self.face = match self.face {
      Side::White => Side::Black,
      Side::Black => Side::White,
    };
  }
}

struct Floor {
  tiles: HashMap<(isize, isize), Tile>,
}
impl Floor {
  fn new() -> Self {
    Floor {
      tiles: HashMap::new(),
    }
  }

  fn walk_and_flip(&mut self, s: &str) {
    let mut chars = s.chars();
    let mut x = 0;
    let mut y = 0;
    loop {
      match chars.next() {
        None => break,
        Some('e') => x += 2,
        Some('w') => x -= 2,
        Some('s') => {
          y += 1;
          match chars.next() {
            Some('e') => x += 1,
            Some('w') => x -= 1,
            _ => panic!("bad s direction"),
          }
        }
        Some('n') => {
          y -= 1;
          match chars.next() {
            Some('e') => x += 1,
            Some('w') => x -= 1,
            _ => panic!("bad n direction"),
          }
        }
        Some(c) => panic!("bad direction {}", c),
      }
    }
    self.flip_tile(x, y);
  }

  fn flip_tile(&mut self, x: isize, y: isize) {
    self.tiles.entry((x, y)).or_insert(Tile::new()).flip();
    for (nx, ny) in [(x-2, y), (x-1, y+1), (x+1, y+1), (x+2, y), (x+1, y-1), (x-1, y-1)].iter() {
      self.tiles.entry((*nx, *ny)).or_insert(Tile::new());
    }
    //println!("Flip ({}, {}) to {:?}", x, y, self.tiles.get(&(x, y)).unwrap().face);
  }

  fn count_neighbours(&self, x: isize, y: isize, face: Side) -> usize {
    let mut count = 0;
    for (nx, ny) in [(x-2, y), (x-1, y+1), (x+1, y+1), (x+2, y), (x+1, y-1), (x-1, y-1)].iter() {
      if let Some(tile) = self.tiles.get(&(*nx, *ny)) {
        if face == tile.face {
          count += 1;
        }
      } else {
        // Not flipped ever, so white.
        if face == Side::White {
          count += 1;
        }
      }
    }
    count
  }

  fn step_game(&mut self) {
    let mut to_flip: Vec<(isize, isize)> = Vec::new();

    for ((x, y), tile) in &self.tiles {
      match tile.face {
        Side::Black => {
          let n = self.count_neighbours(*x, *y, Side::Black);
          if n == 0 || n > 2 {
            to_flip.push((*x, *y));
          }
        },
        Side::White => {
          let n = self.count_neighbours(*x, *y, Side::Black);
          if n == 2 {
            to_flip.push((*x, *y));
          }
        },
      }
    }

    for (x, y) in to_flip {
      self.flip_tile(x, y);
    }
  }

  fn count_tiles(&self, face: Side) -> usize {
    self
      .tiles
      .iter()
      .filter(|(_, tile)| tile.face == face)
      .count()
  }
}

fn solve(input_all: String) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();

  let mut floor = Floor::new();
  for line in &lines {
    floor.walk_and_flip(line);
  }
  println!("Part 1 {}", floor.count_tiles(Side::Black));

  for _day in 1..=100 {
    floor.step_game();
    //println!("Day {}: {}", _day, floor.count_tiles(Side::Black));
  }
  println!("Part 2 {}", floor.count_tiles(Side::Black));

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = if std::env::args().nth(1).filter(|s| s == "test").is_some() {
    std::fs::read_to_string("day24/test.txt")?
  } else {
   std::fs::read_to_string("day24/input.txt")?
  };
  solve(input_all)
}
