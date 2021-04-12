static INPUT_FILE: &str = "day3/input.txt";

#[derive(Eq, PartialEq)]
enum MapSpot {
  Open,
  Tree,
}

impl std::fmt::Display for MapSpot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MapSpot::Open => "Open",
        MapSpot::Tree => "Tree",
      }
    )
  }
}

fn read_file_lines(name: &str) -> Vec<String> {
  let in_str: String = match std::fs::read_to_string(name) {
    Ok(s) => s,
    Err(e) => panic!("{}", e),
  };
  return in_str.split_terminator('\n').map(String::from).collect();
}

fn map_spot(x: usize, y: usize, map: &Vec<String>) -> MapSpot {
  const LINELEN : usize = 31;
  let line = map.iter().nth(y).unwrap();
  let ch = line.chars().nth(x % LINELEN).unwrap();
  match ch {
    '.' => MapSpot::Open,
    '#' => MapSpot::Tree,
    _ => panic!("bad input at {},{}", x, y),
  }
}

fn main() {
  let lines: Vec<String> = read_file_lines(INPUT_FILE);

  const XDELTA: usize = 3;
  {
    let mut x = 0;
    let mut trees: usize = 0;
    for y in 0..lines.len() {
      trees += (map_spot(x, y, &lines) == MapSpot::Tree) as usize;
      x += XDELTA;
    }
    println!("Part 1 {}", trees);
  }
  {
    let lines_iter = lines.iter().enumerate();
    let trees = lines_iter.fold(0, |acc, (y, _)| {
      acc + (map_spot(y * XDELTA, y, &lines) == MapSpot::Tree) as usize
    });
    println!("Part 1 {}", trees);
  }

  {
    let mut acc = 1;
    for (xdelta, ydelta) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
      let mut x = 0;
      let mut trees: usize = 0;
      for y in (0..lines.len()).step_by(*ydelta) {
        trees += (map_spot(x, y, &lines) == MapSpot::Tree) as usize;
        x += xdelta;
      }
      acc *= trees;
    }
    println!("Part 2 {}", acc);
  }
  {
    let mut acc = 1;
    for (xdelta, ydelta) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
      let lines_iter = lines.iter().enumerate().step_by(*ydelta);
      let trees_iter =
          lines_iter.filter(|(y, _)| map_spot(y * xdelta, *y, &lines) == MapSpot::Tree);
      acc *= trees_iter.count();
    }
    println!("Part 2 {}", acc);
  }
}
