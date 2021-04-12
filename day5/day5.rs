extern crate regex;
use regex::Regex;

static INPUT_FILE: &str = "day5/input.txt";

fn main() {
  let lines: String = std::fs::read_to_string(INPUT_FILE).unwrap();

  let mut max_seat_id = 0;
  let mut seats = std::collections::HashSet::<u32>::new();

  let re = Regex::new(r"\b([FB]{7})([LR]{3})\b").unwrap();
  for c in re.captures_iter(&lines) {
    let mut row_fb : String = String::from(c.get(1).unwrap().as_str());
    let mut col_fb : String = String::from(c.get(2).unwrap().as_str());
    row_fb = row_fb.chars().map(|c| if c == 'B' {'1'} else{'0'}).collect();
    col_fb = col_fb.chars().map(|c| if c == 'R' {'1'} else{'0'}).collect();
    let row_num = u32::from_str_radix(&row_fb, 2).unwrap();
    let col_num = u32::from_str_radix(&col_fb, 2).unwrap();
    let seat_id = row_num * 8 + col_num;
    max_seat_id = std::cmp::max(seat_id, max_seat_id);
    seats.insert(seat_id);
  }

  println!("Max Seat {}", max_seat_id);

  let mut before = seats.get(&0).is_some();
  let mut mine = seats.get(&1).is_some();
  for i in 1..2_u32.pow(10)-1 {
    let after = seats.get(&(i+1)).is_some();
    if before && after && !mine {
      println!("My seat: {}", i);
      break;
    }
    before = mine;
    mine = after;
  }
}
