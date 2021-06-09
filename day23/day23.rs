use std::ptr::null_mut;

struct Cup {
  label: usize,
  next: *mut Cup,
  picked_up: bool,
}
impl Cup {
  fn new(label: usize) -> Cup {
    Cup {
      label: label,
      next: null_mut(),
      picked_up: false,
    }
  }
}

struct Cups {
  current: *mut Cup,
  pickup: Option<[*mut Cup; 3]>,
  // For each Cup label N, index [N-1] points to that Cup.
  map: Vec<*mut Cup>,
  max_label: usize,
}
impl Cups {
  fn make_cups(order: &str) -> Self {
    assert!(order.chars().count() > 3); // Else pickup() would steal `self.current`.
    let mut cups = Cups {
      current: null_mut(),
      pickup: None,
      map: Vec::new(),
      max_label: 0,
    };
    cups.map.resize(order.len(), null_mut());
    let mut cups_it = order
      .chars()
      .map(|c| c.to_string().parse::<usize>().unwrap())
      .rev();
    while let Some(label) = cups_it.next() {
      let new_cup = Box::into_raw(Box::new(Cup::new(label)));
      cups.map[label - 1] = new_cup;
      cups.max_label = std::cmp::max(cups.max_label, label);
      unsafe {
        (*new_cup).next = cups.current;
      }
      cups.current = new_cup;
    }
    // Make the tail wrap around.
    unsafe {
      let mut tail = cups.current;
      while !(*tail).next.is_null() {
        tail = (*tail).next
      }
      assert!((*tail).next.is_null());
      (*tail).next = cups.current;
    }
    cups
  }

  fn make_cups_until(order: &str, until: usize) -> Self {
    let mut cups = Self::make_cups(order);
    cups.map.resize(until, null_mut());
    let mut tail = cups.current;
    unsafe {
      while (*tail).next != cups.current {
        tail = (*tail).next;
      }
    }

    // Make the ordered nodes.
    let mut ordered_head: *mut Cup = null_mut();
    let mut ordered_tail: *mut Cup = null_mut();
    let mut max = cups.max_label;
    while max < until {
      let label = max + 1;
      let new_cup = Box::into_raw(Box::new(Cup::new(label)));
      if ordered_head.is_null() {
        ordered_head = new_cup;
      }
      if !ordered_tail.is_null() {
        unsafe {
          (*ordered_tail).next = new_cup;
        }
      }
      ordered_tail = new_cup;
      cups.map[label - 1] = new_cup;
      max = label
    }
    // Insert them into the ring.
    unsafe {
      (*tail).next = ordered_head;
      (*ordered_tail).next = cups.current;
    }
    cups.max_label = max;
    cups
  }

  // Pulls the 3 nodes after current out of the list.
  fn pickup_after_current(&mut self) {
    assert!(self.pickup.is_none());
    let mut pickup = [null_mut(); 3];
    unsafe {
      let mut p = (*self.current).next;
      for i in 0..3 {
        pickup[i] = p;
        (*p).picked_up = true;
        p = (*p).next;
      }
      (*self.current).next = p;
    }
    self.pickup = Some(pickup);
  }

  // Inserts the `self.pickup` nodes after `destination`.
  fn insert_pickup_after(&mut self, destination: *mut Cup) {
    assert!(!destination.is_null());
    let pickup = &self.pickup.unwrap();
    unsafe {
      let old_next = (*destination).next;
      // Walk the pickups, inserting them back into the map, then connect
      // the old neighbour of destination afterward.
      (*destination).next = pickup[0];
      for i in 0..3 {
        (*pickup[i]).picked_up = false
      }
      (*pickup[2]).next = old_next;
    }
    self.pickup = None;
  }

  fn move_current(&mut self) {
    unsafe {
      self.current = (*self.current).next;
    }
  }

  fn find_destination(&self) -> *mut Cup {
    let mut search = unsafe { (*self.current).label };
    loop {
      if search == 1 {
        // There's no cup labeled 0, so we can wrap around here.
        search = self.max_label;
      } else {
        search -= 1;
      }
      let cup = self.map[search - 1];
      if !unsafe { (*cup).picked_up } {
        return cup;
      }
    }
  }

  fn next(&self, label: usize) -> usize {
    let p = self.map[label - 1];
    unsafe {
      let p_next = (*p).next;
      (*p_next).label
    }
  }

  #[allow(dead_code)]
  fn cups_as_string(&self) -> String {
    let mut s = String::new();
    let mut p = self.current;
    loop {
      let label = unsafe { (*p).label };

      if p != self.current {
        s.push_str(", ");
      }
      if p == self.current {
        s.push('(');
      }
      s.push_str(&label.to_string());
      if p == self.current {
        s.push(')');
      }

      unsafe { p = (*p).next };
      if p == self.current {
        break;
      }
    }
    s
  }

  #[allow(dead_code)]
  fn pickup_as_string(&self) -> String {
    let mut s = String::new();
    if let Some(pickup) = self.pickup {
      for i in 0..3 {
        if i > 0 {
          s.push_str(", ");
        }
        let label = unsafe { &(*pickup[i]).label };
        s.push_str(&label.to_string());
      }
    }
    s
  }
}

fn p1(input_all: &str) {
  let mut cups = Cups::make_cups(input_all);
  for _step in 1..=100 {
    //println!("-- move {} --", _step);
    //println!("cups: {}", cups.cups_as_string());
    cups.pickup_after_current();
    let dest = cups.find_destination();
    //println!("pick up: {}", cups.pickup_as_string());
    //println!("destination: {}\n", dest);
    cups.insert_pickup_after(dest);
    cups.move_current();
  }
  //println!("-- final --");
  //println!("cups: {}\n", cups.cups_as_string());
  let p1 = {
    let mut s = String::new();
    let mut n = 1;
    loop {
      n = cups.next(n);
      if n == 1 {
        break s;
      }
      s.push_str(&n.to_string());
    }
  };
  println!("Part 1 {}", p1);
}

fn p2(input_all: &str) {
  let mut cups = Cups::make_cups_until(input_all, 1_000_000);
  for _step in 1..=10_000_000 {
    //println!("-- move {} --", _step);
    //println!("cups: {}", cups.cups_as_string());
    cups.pickup_after_current();
    let dest = cups.find_destination();
    //println!("pick up: {}", cups.pickup_as_string());
    //println!("destination: {}\n", dest);
    cups.insert_pickup_after(dest);
    cups.move_current();
  }
  //println!("-- final --");
  //println!("cups: {}\n", cups.cups_as_string());
  let n1 = cups.next(1);
  let n2 = cups.next(n1);
  let p2 = n1 as u64 * n2 as u64;
  println!("Part 2 {}", p2);
}

fn main() {
  let input_all = String::from("716892543");
  //let input_all = String::from("389125467"); // Test input.
  p1(&input_all);
  p2(&input_all);
}
