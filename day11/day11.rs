static INPUT_FILE : &str = "day11/input.txt";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spot {
  Chair(bool),
  Floor,
}

const W : usize = 99;
const H : usize = 95;

#[derive(Clone)]
struct World {
  spots: [Spot; W * H],
}
impl World {
  fn from_string(input: &str) -> Self {
    let mut w = World {spots: [Spot::Floor; W * H]};
    let mut input_iter = input.chars().filter(|&x| x == '#' || x == 'L' || x == '.');
    for s in &mut w.spots {
      *s = match input_iter.next().unwrap() as char {
        '#' => Spot::Chair(true),
        'L' => Spot::Chair(false),
        '.' => Spot::Floor,
        _  => panic!("Unexpected input character\n")
      }
    }
    assert!(input_iter.next().is_none());
    w
  }

  fn get(&self, x: usize, y: usize) -> Spot {
    assert!(x < W);
    assert!(y < H);
    self.spots[x + y * W]
  }

  fn set(&mut self, x: usize, y: usize, spot: Spot) {
    assert!(x < W);
    assert!(y < H);
    self.spots[x + y * W] = spot;
  }

  fn is_empty_chair(&self, x: usize, y: usize) -> bool {
    self.get(x, y) == Spot::Chair(false)
  }

  fn is_full_chair(&self, x: usize, y: usize) -> bool {
    self.get(x, y) == Spot::Chair(true)
  }

  fn count_neighbours(&self, x: usize, y: usize) -> usize {
    let mut count = 0;
    for xd in x.saturating_sub(1)..std::cmp::min(W, x+2) {
      for yd in y.saturating_sub(1)..std::cmp::min(H, y+2) {
        if (xd != x || yd != y) && self.get(xd, yd) == Spot::Chair(true) {
          count += 1;
        }
      }
    }
    count
  }

  fn count_sight_neighbours(&self, x: usize, y: usize) -> usize {
    let mut count = 0;
    // Up.
    for yd in (0..y).rev() {
      if let Spot::Chair(full) = self.get(x, yd)  {
        count += full as usize;
        break;
      }
    }
    // Down.
    for yd in y+1..H {
      if let Spot::Chair(full) = self.get(x, yd)  {
        count += full as usize;
        break;
      }
    }
    // Left.
    for xd in (0..x).rev() {
      if let Spot::Chair(full) = self.get(xd, y)  {
        count += full as usize;
        break;
      }
    }
    // Right.
    for xd in x+1..W {
      if let Spot::Chair(full) = self.get(xd, y)  {
        count += full as usize;
        break;
      }
    }
    // Up-Left.
    for (xd, yd) in (0..x).rev().zip((0..y).rev()) {
      if let Spot::Chair(full) = self.get(xd, yd)  {
        count += full as usize;
        break;
      }
    }
    // Up-Right.
    for (xd, yd) in (x+1..W).zip((0..y).rev()) {
      if let Spot::Chair(full) = self.get(xd, yd)  {
        count += full as usize;
        break;
      }
    }
    // Down-Left.
    for (xd, yd) in (0..x).rev().zip(y+1..H) {
      if let Spot::Chair(full) = self.get(xd, yd)  {
        count += full as usize;
        break;
      }
    }
    // Down-Right.
    for (xd, yd) in (x+1..W).zip(y+1..H) {
      if let Spot::Chair(full) = self.get(xd, yd)  {
        count += full as usize;
        break;
      }
    }
    count
  }

  fn count_full_chairs(&self) -> usize {
        self.spots.iter().filter(|&&x| x == Spot::Chair(true)).count()
  }

  fn print(&self) {
    for y in 0..H {
      for x in 0..W {
        let c = match self.get(x, y) {
            Spot::Chair(true) => '#',
            Spot::Chair(false) => 'L',
            Spot::Floor => '.',
        };
        print!("{}", c);
      }
      println!("");
    }
    }
}

fn main() {
  let file = std::fs::read_to_string(INPUT_FILE).unwrap();

  let mut changed = false;
  let mut world = World::from_string(&file);
  loop {
    let mut next_world = world.clone();
    for x in 0..W {
      for y in 0..H {
        if world.is_empty_chair(x, y) && world.count_neighbours(x, y) == 0 {
          next_world.set(x, y, Spot::Chair(true));
          changed = true;
        } else if world.is_full_chair(x, y) && world.count_neighbours(x, y) >= 4 {
          next_world.set(x, y, Spot::Chair(false));
          changed = true;
        }
      }
    }

    world = next_world;
    if !changed { break; }
    changed = false;
  }
  println!("Part 1 {}", world.count_full_chairs());

  world = World::from_string(&file);
  changed = false;
  loop {
    let mut next_world = world.clone();
    for x in 0..W {
      for y in 0..H {
        if world.is_empty_chair(x, y) && world.count_sight_neighbours(x, y) == 0 {
          next_world.set(x, y, Spot::Chair(true));
          changed = true;
        } else if world.is_full_chair(x, y) && world.count_sight_neighbours(x, y) >= 5 {
          next_world.set(x, y, Spot::Chair(false));
          changed = true;
        }
      }
    }

    world = next_world;
    if !changed { break; }
    changed = false;
  }
  println!("Part 2 {}", world.count_full_chairs());
}