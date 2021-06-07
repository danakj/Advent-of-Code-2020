// #[macro_use]
extern crate anyhow;
extern crate regex;
use regex::Regex;
extern crate itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const TILE_SIZE: usize = 10;
const INNER_SIZE: usize = TILE_SIZE - 2;
// 4 rotations with front face up, 4 rotations with back face up.
const NUM_ORIENTATIONS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
  Rot90,
  FlipHorz,
}

#[derive(Clone, Debug)]
struct Image {
  width: usize,
  height: usize, // Same as width.
  bitmap: Vec<bool>,
}
impl Image {
  fn orient_rel(&mut self, op: Operation) {
    assert_eq!(self.width, self.height);

    match op {
      Operation::Rot90 => {
        let mut out = self.bitmap.clone();
        for x in 0..self.width {
          for y in 0..self.height {
            let out_x = self.height - y - 1;
            let out_y = x;
            out[out_y * self.width + out_x] = self.bitmap[y * self.width + x]
          }
        }
        self.bitmap = out;
      }
      Operation::FlipHorz => {
        for y in 0..self.height {
          for x in 0..self.width / 2 {
            let at1 = y * self.width + x;
            let at2 = y * self.width + (self.width - 1 - x);
            let s = self.bitmap[at1];
            self.bitmap[at1] = self.bitmap[at2];
            self.bitmap[at2] = s;
          }
        }
      }
    }
  }

  fn get_row(&self, row: usize) -> &[bool] {
    let start = row * self.width;
    let end = (row + 1) * self.width;
    assert!(row < self.height);
    assert!(end <= self.width * self.height);
    &self.bitmap[start..end]
  }

  fn get_row_string(&self, row: usize) -> String {
    let mut s = String::new();
    for b in self.get_row(row) {
      match b {
        true => s.push('#'),
        false => s.push('.'),
      }
    }
    s
  }
}
impl std::fmt::Display for Image {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut i = 0;
    for _ in 0..self.height {
      for _ in 0..self.width {
        write!(f, "{}", if self.bitmap[i] { '#' } else { '.' })?;
        i += 1;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct EdgeNum(u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Edge {
  num: EdgeNum,
  flipped: bool,
  border: bool,
}
impl Edge {
  fn new(s: &str, flipped: bool) -> Self {
    let r = s.chars().rev().collect::<String>();
    let num = u32::from_str_radix(s, 2).unwrap();
    let reverse = u32::from_str_radix(&r, 2).unwrap();
    // Use the lowest number for the edge, so that they can be compared for uniqueness.
    // If the flipped version is lower, then use that and mark the edge flipped.
    if num < reverse {
      Edge {
        num: EdgeNum(num),
        flipped: flipped,
        border: true,
      }
    } else {
      Edge {
        num: EdgeNum(reverse),
        flipped: !flipped,
        border: true,
      }
    }
  }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct TileId(u64);

#[derive(Clone)]
struct Tile {
  id: TileId,
  top: Edge,
  left: Edge,
  right: Edge,
  bottom: Edge,
  // The image for the tile, excluding the outer row/columns.
  inner_image: Image,
  // The set of ops performed on the tile, but not applied to the `inner_image` yet.
  image_ops: Vec<Operation>,
}
impl Tile {
  fn from_strings(lines: Vec<&str>) -> Vec<Tile> {
    let all_tile_strs: Vec<String> = {
      let it = lines.into_iter().filter(|&s| s != "");
      let it = it.map(|s| s.replace("#", "1").replace(".", "0").to_owned());
      it.collect()
    };

    let mut tiles = Vec::new();
    // Each `tile_strs` is the input lines for a single tile. The first line
    // is the id, the next TILE_SIZE lines are the content of the tile's image.
    for tile_strs in all_tile_strs.chunks_exact(TILE_SIZE + 1) {
      // Format of tile_strs[0] is `Tile <id>:`.
      let id = {
        let id_re = Regex::new("[0-9]+").unwrap();
        TileId(id_re.find(&tile_strs[0]).unwrap().as_str().parse().unwrap())
      };
      let top = &tile_strs[1];
      let bottom = &tile_strs[TILE_SIZE];
      let left = tile_strs[1..=TILE_SIZE]
        .iter()
        .fold(String::new(), |mut acc, s| {
          acc.push(s.chars().nth(0).unwrap());
          acc
        });
      let right = tile_strs[1..=TILE_SIZE]
        .iter()
        .fold(String::new(), |mut acc, s| {
          acc.push(s.chars().nth(TILE_SIZE - 1).unwrap());
          acc
        });
      // Construct an Image bitmap (bools) from the tile, excluding the outer row/columns.
      let inner_bitmap = {
        let mut v = Vec::new();
        for row in 2..2 + INNER_SIZE {
          let str = &tile_strs[row];
          for char in str[1..=INNER_SIZE].chars() {
            v.push(match char {
              '1' => true,
              _ => false,
            });
          }
        }
        v
      };

      tiles.push(Tile {
        id: id,
        top: Edge::new(top, false),
        right: Edge::new(&right, false),
        bottom: Edge::new(bottom, true),
        left: Edge::new(&left, true),
        inner_image: Image {
          width: INNER_SIZE,
          height: INNER_SIZE,
          bitmap: inner_bitmap,
        },
        image_ops: Vec::new(),
      });

      // Find unique borders.
      let mut all_edges: Vec<&mut Edge> = Vec::new();
      for t in &mut tiles {
        all_edges.push(&mut t.left);
        all_edges.push(&mut t.top);
        all_edges.push(&mut t.right);
        all_edges.push(&mut t.bottom);
      }
      for i in 0..all_edges.len() {
        for j in i + 1..all_edges.len() {
          if all_edges[i].num == all_edges[j].num {
            all_edges[i].border = false;
            all_edges[j].border = false;
            break;
          }
        }
      }
    }
    tiles
  }

  fn orient_rel(&mut self, op: Operation) {
    self.image_ops.push(op);
    let left = self.left;
    let top = self.top;
    let right = self.right;
    let bottom = self.bottom;
    match op {
      Operation::Rot90 => {
        self.left = bottom;
        self.bottom = right;
        self.right = top;
        self.top = left;
      }
      Operation::FlipHorz => {
        self.left = right;
        self.right = left;
        self.bottom.flipped = !self.bottom.flipped;
        self.top.flipped = !self.top.flipped;
        self.left.flipped = !self.left.flipped;
        self.right.flipped = !self.right.flipped;
      }
    }
  }

  fn num_border_edges(&self) -> usize {
    let edges = [&self.left, &self.right, &self.top, &self.bottom];
    let it = edges.iter().filter(|&&edge| edge.border);
    it.count()
  }

  fn resolve_image(&mut self) {
    for op in &self.image_ops {
      self.inner_image.orient_rel(*op);
    }
    self.image_ops.clear();
  }
}

fn solve(input_all: &str) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let tiles = Tile::from_strings(lines);

  let num_tiles = tiles.len();
  let board_width = {
    let mut i = 1;
    loop {
      if i * i == num_tiles {
        break i;
      }
      i += 1;
      assert!(i < num_tiles / 2);
    }
  };

  let corners = {
    let mut v = Vec::new();
    for t in &tiles {
      if t.num_border_edges() == 2 {
        v.push(t.clone());
      }
    }
    v
  };
  assert_eq!(corners.len(), 4);
  let edges = {
    let mut v = Vec::new();
    for t in &tiles {
      if t.num_border_edges() == 1 {
        v.push(t.clone());
      }
    }
    v
  };
  assert_eq!(edges.len(), (board_width - 2) * 4);
  let inners = {
    let mut v = Vec::new();
    for t in &tiles {
      if t.num_border_edges() == 0 {
        v.push(t.clone());
      }
    }
    v
  };
  assert_eq!(inners.len(), (board_width - 2) * (board_width - 2));

  fn collect_tile_edges(tile_list: &Vec<Tile>) -> HashMap<EdgeNum, Vec<&Tile>> {
    let mut h = HashMap::new();
    for tile in tile_list.iter() {
      for edge in [&tile.top, &tile.left, &tile.right, &tile.bottom].iter() {
        match h.get_mut(&edge.num) {
          None => drop(h.insert(edge.num, vec![tile])),
          Some(tiles_vec) => tiles_vec.push(tile),
        };
      }
    }
    h
  }

  let corner_tiles_by_edge = collect_tile_edges(&corners);
  let edge_tiles_by_edge = collect_tile_edges(&edges);
  let inner_tiles_by_edge = collect_tile_edges(&inners);
  let tiles_by_id: HashMap<TileId, &Tile> = {
    let mut h = HashMap::new();
    for tile in &tiles {
      h.insert(tile.id, tile);
    }
    h
  };

  #[derive(Clone)]
  struct Ctx<'a> {
    board_width: usize,
    tiles_by_id: &'a HashMap<TileId, &'a Tile>,
    corner_tiles_by_edge: &'a HashMap<EdgeNum, Vec<&'a Tile>>,
    edge_tiles_by_edge: &'a HashMap<EdgeNum, Vec<&'a Tile>>,
    inner_tiles_by_edge: &'a HashMap<EdgeNum, Vec<&'a Tile>>,
    board: Vec<Tile>,
    used_tiles: HashSet<TileId>,
  }
  let mut ctx = Ctx {
    board_width: board_width,
    tiles_by_id: &tiles_by_id,
    corner_tiles_by_edge: &corner_tiles_by_edge,
    edge_tiles_by_edge: &edge_tiles_by_edge,
    inner_tiles_by_edge: &inner_tiles_by_edge,
    board: Vec::new(),
    used_tiles: HashSet::new(),
  };

  let mut top_left = corners[0].clone();
  while !top_left.left.border || !top_left.top.border {
    top_left.orient_rel(Operation::Rot90);
  }
  ctx.used_tiles.insert(top_left.id);
  ctx.board.push(top_left);

  fn try_tile(at: usize, ctx: Ctx) -> Option<Vec<Tile>> {
    let board_width = ctx.board_width;
    let x = at % board_width;
    let y = at / board_width;

    if at == board_width * board_width {
      return Some(ctx.board.clone());
    }
    let all_candidates = {
      if (x == 0 && y == board_width - 1)
        || (x == board_width - 1 && y == 0)
        || (x == board_width - 1 && y == board_width - 1)
      {
        &ctx.corner_tiles_by_edge
      } else if x == 0 || y == 0 || x == board_width - 1 || y == board_width - 1 {
        &ctx.edge_tiles_by_edge
      } else {
        &ctx.inner_tiles_by_edge
      }
    };

    // Find the set of candidate tiles that can be placed at position `at` such that
    // they have an edge that matches the tile above them and to the left of them.
    // This has to handle the edge cases of tiles in the top row/left column, where
    // there's nothing to match there so any edge works. Tile at (0,0) is placed first
    // before this function is ever called, so it doesn't need to choose that one.
    let mut candidates: HashSet<TileId> = HashSet::new();
    match (x, y) {
      (0, 0) => panic!("we don't see 0,0"),
      (0, _) => {
        let top = ctx.board[(y - 1) * board_width + x].bottom;
        for candi in &all_candidates[&top.num] {
          if !ctx.used_tiles.contains(&candi.id) {
            candidates.insert(candi.id);
          }
        }
      }
      (_, 0) => {
        let left = ctx.board[y * board_width + x - 1].right;
        for candi in &all_candidates[&left.num] {
          if !ctx.used_tiles.contains(&candi.id) {
            candidates.insert(candi.id);
          }
        }
      }
      (_, _) => {
        let left = ctx.board[y * board_width + x - 1].right;
        let top = ctx.board[(y - 1) * board_width + x].bottom;
        let mut left_candidates: HashSet<TileId> = HashSet::new();
        for candi in &all_candidates[&left.num] {
          left_candidates.insert(candi.id);
        }
        for candi in &all_candidates[&top.num] {
          if left_candidates.contains(&candi.id) {
            if !ctx.used_tiles.contains(&candi.id) {
              candidates.insert(candi.id);
            }
          }
        }
      }
    };

    for candi in candidates {
      let mut tile = ctx.tiles_by_id[&candi].clone();

      for i in 0..NUM_ORIENTATIONS {
        let recurse = match (x, y) {
          (0, 0) => panic!("we don't see 0,0"),
          (0, _) => {
            let top = ctx.board[(y - 1) * board_width + x].bottom;
            tile.top.num == top.num && tile.top.flipped != top.flipped
          }
          (_, 0) => {
            let left = ctx.board[y * board_width + x - 1].right;
            tile.left.num == left.num && tile.left.flipped != left.flipped
          }
          (_, _) => {
            let left = ctx.board[y * board_width + x - 1].right;
            let top = ctx.board[(y - 1) * board_width + x].bottom;
            tile.top.num == top.num
              && tile.top.flipped != top.flipped
              && tile.left.num == left.num
              && tile.left.flipped != left.flipped
          }
        };
        if recurse {
          // Match! Try recurse.
          let mut ctx = ctx.clone();
          ctx.board.push(tile.clone());
          ctx.used_tiles.insert(tile.id);
          if let Some(board) = try_tile(at + 1, ctx) {
            return Some(board);
          }
        }

        // Flip `tile` to next orientation.
        if i == NUM_ORIENTATIONS / 2 - 1 {
          tile.orient_rel(Operation::Rot90); // Back to 0deg.
          tile.orient_rel(Operation::FlipHorz);
        } else {
          tile.orient_rel(Operation::Rot90);
        }
      }
    }

    None
  }

  let solved_board_result = try_tile(1, ctx);
  let mut board = solved_board_result.expect("Failed to find a board layout");

  let m = board_width - 1;
  let a = 0 + board_width * 0;
  let b = m + board_width * 0;
  let c = 0 + board_width * m;
  let d = m + board_width * m;
  println!(
    "Part 1 {}",
    board[a].id.0 * board[b].id.0 * board[c].id.0 * board[d].id.0
  );

  for tile in &mut board {
    tile.resolve_image();
  }

  // Construct the final Image from the combined tiles' inner images.
  let mut image = Image {
    width: board_width * INNER_SIZE,
    height: board_width * INNER_SIZE,
    bitmap: Vec::new(),
  };
  image
    .bitmap
    .reserve(board_width * board_width * INNER_SIZE * INNER_SIZE);
  for tile_y in 0..board_width {
    for tile_row in 0..INNER_SIZE {
      for tile_x in 0..board_width {
        let at = tile_y * board_width + tile_x;
        let tile_bitmap = &board[at].inner_image.bitmap;
        let start = tile_row * INNER_SIZE;
        let end = (tile_row + 1) * INNER_SIZE;
        for i in start..end {
          image.bitmap.push(tile_bitmap[i]);
        }
      }
    }
  }
  assert_eq!(
    image.bitmap.len(),
    board_width * board_width * INNER_SIZE * INNER_SIZE
  );

  // Regexes use ^ because we may need to find overlapping patterns, so we will
  // be searching at each position in the image strings.
  let monster = [
    Regex::new(r"^..................#.").unwrap(),
    Regex::new(r"^#....##....##....###").unwrap(),
    Regex::new(r"^.#..#..#..#..#..#...").unwrap(),
  ];
  // The length of each monster line pattern.
  const MONSTER_LEN: usize = 20;
  // How many tiles the monster covers. The number of # in the monster pattern above.
  const MONSTER_COVERAGE: usize = 15;

  for i in 0..NUM_ORIENTATIONS {
    // Look for sea monsters.
    let mut num_monsters = 0;
    for row in 0..image.height - 2 {
      let get_monster_positions = |monster_re: &Regex, row: usize| -> Vec<usize> {
        let row_str = image.get_row_string(row);
        let mut v = Vec::new();
        for i in 0..row_str.len() - MONSTER_LEN {
          if let Some(m) = monster_re.find(&row_str[i..]) {
            assert_eq!(m.start(), 0);
            v.push(i);
          }
        }
        v
      };
      let mut first = get_monster_positions(&monster[0], row);
      let mut second = get_monster_positions(&monster[1], row + 1);
      let mut third = get_monster_positions(&monster[2], row + 2);

      // Count matches.
      while !first.is_empty() && !second.is_empty() && !third.is_empty() {
        if first[0] == second[0] && second[0] == third[0] {
          num_monsters += 1;
          first.remove(0);
          second.remove(0);
          third.remove(0);
        } else if first[0] <= second[0] && first[0] <= third[0] {
          first.remove(0);
        } else if second[0] <= first[0] && second[0] <= third[0] {
          second.remove(0);
        } else {
          assert!(third[0] <= first[0] && third[0] <= second[0]);
          third.remove(0);
        }
      }
    }
    if num_monsters > 0 {
      let wave_count = image.bitmap.iter().filter(|&b| *b).count();
      println!("Part 2 {}", wave_count - num_monsters * MONSTER_COVERAGE);
      break;
    }

    // Flip `image` to next orientation.
    if i == NUM_ORIENTATIONS / 2 - 1 {
      image.orient_rel(Operation::Rot90); // Back to 0deg.
      image.orient_rel(Operation::FlipHorz);
    } else {
      image.orient_rel(Operation::Rot90);
    }
  }
  Ok(())
}

fn main() -> anyhow::Result<()> {
  //let input_all = std::fs::read_to_string("day20/test1.txt")?;
  let input_all = std::fs::read_to_string("day20/input.txt")?;
  solve(&input_all)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_mutation() {
    #![allow(dead_code)]

    const BASE: &str = r"Tile 2953:
      ##########
      ..........
      ##########
      ..........
      ##########
      #......#..
      #......#..
      #......#..
      #......#..
      #......#..";
    const ROTATE1: &str = r"Tile 2953:
      ######.#.#
      .....#.#.#
      .....#.#.#
      .....#.#.#
      .....#.#.#
      .....#.#.#
      .....#.#.#
      ######.#.#
      .....#.#.#
      .....#.#.#";
    const ROTATE2: &str = r"Tile 2953:
      ..#......#
      ..#......#
      ..#......#
      ..#......#
      ..#......#
      ##########
      ..........
      ##########
      ..........
      ##########";
    const FLIPPED: &str = r"Tile 2953:
      ##########
      ..........
      ##########
      ..........
      ##########
      ..#......#
      ..#......#
      ..#......#
      ..#......#
      ..#......#";

    let base = &Tile::from_strings(BASE.split_terminator("\n").map(|s| s.trim()).collect())[0];
    let expect_rotate1 =
      &Tile::from_strings(ROTATE1.split_terminator("\n").map(|s| s.trim()).collect())[0];
    let expect_rotate2 =
      &Tile::from_strings(ROTATE2.split_terminator("\n").map(|s| s.trim()).collect())[0];
    let expect_flipped =
      &Tile::from_strings(FLIPPED.split_terminator("\n").map(|s| s.trim()).collect())[0];

    let compare_tile_ops = |base: &Tile, ops: Vec<Operation>, expect: &Tile| {
      let mut actual = base.clone();
      for op in &ops {
        actual.orient_rel(*op);
      }
      assert_eq!(actual.image_ops, ops);
      actual.resolve_image();
      assert_eq!(actual.image_ops, vec![]);

      if actual.inner_image.bitmap != expect.inner_image.bitmap {
        println!("EXPECT\n{}", expect.inner_image);
        println!("ACTUAL\n{}", actual.inner_image);
        assert_eq!(actual.inner_image.bitmap, expect.inner_image.bitmap);
      }
    };

    compare_tile_ops(base, vec![Operation::Rot90], expect_rotate1);
    compare_tile_ops(
      base,
      vec![Operation::Rot90, Operation::Rot90],
      expect_rotate2,
    );
    compare_tile_ops(base, vec![Operation::FlipHorz], expect_flipped);
  }
}
