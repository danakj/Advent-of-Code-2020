#[macro_use]
extern crate anyhow;

static INPUT_FILE: &str = "day13/input.txt";

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string(INPUT_FILE)?;
  //let input_all = "1\n67,7,59,61";

  let (start_time, bus_ids) = {
    let mut iter = input_all.split_terminator("\n");
    let start_time = iter.next().ok_or(anyhow!("No leave time"))?.parse::<i64>()?;
    let bus_ids : Vec<_> = iter.next().ok_or(anyhow!("No bus ids"))?.split_terminator(",").map(|x| x.parse::<i64>().ok()).collect();
    (start_time, bus_ids)
  };

  p1(start_time, &bus_ids)?;
  p2(start_time, &bus_ids)?;
  Ok(())
}

fn p1(start_time: i64, bus_ids: &Vec<Option<i64>>) -> anyhow::Result<()> {
  let mut bus_times: Vec<(i64, i64)> = bus_ids.iter().filter_map(|&opt| opt.map(|i| (i, i))).collect();
  loop {
    let mut changed = false;
    for b in &mut bus_times {
      if b.1 < start_time {
        b.1 += b.0;
        changed = true;
      }
    }
    if !changed { break; }
  }

  bus_times.sort_by_key(|bus| bus.1);  // Sort by time the bus leaves.
  let chosen_bus = &bus_times[0];
  println!("Part 1 {}", chosen_bus.0 * (chosen_bus.1 - start_time));

  Ok(())
}

fn merge_buses(cycle_start: i64, cycle_len: i64, offset_id2: (i64, i64)) -> (i64, i64) {
  let (offset, id2) = offset_id2;
  
  let mut start_time = cycle_start + offset;
  // If time % id2 is the amount of time in the route at `time`
  // that has passed since the `id2` bus left. id2 subtract that
  // time is the time until it gets back. If it gets back at `offset`
  // then it fits the schedule.
  let time_left = |time, bus_id| {
    if time % bus_id == 0 {
      0
    } else {
      bus_id - time % bus_id
    }
  };
  while time_left(start_time, id2) != 0 {
    start_time += cycle_len;
  }
  let mut end_time = start_time + cycle_len;
  while time_left(end_time, id2) != 0 {
    end_time += cycle_len;
  }
  (start_time-offset, end_time-start_time)
}

fn p2(_start_time: i64, bus_ids: &Vec<Option<i64>>) -> anyhow::Result<()> {
  let indexed_bus_ids = bus_ids.iter().enumerate().filter_map(|(i, maybe_id)| maybe_id.map(|id| (i, id))).collect::<Vec<_>>();
  let mut start_time = 0;
  let mut cycle_len = 1;
  for (i, maybe_id) in indexed_bus_ids {
    let (start, len) = merge_buses(start_time, cycle_len, (i as i64, maybe_id));
    start_time = start;
    cycle_len = len;
  }

  println!("Part 2 {}", start_time);
  Ok(())
}