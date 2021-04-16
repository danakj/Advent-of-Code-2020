#[macro_use]
extern crate anyhow;

static INPUT_FILE: &str = "day12/input.txt";

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string(INPUT_FILE)?;
  //let input_all = r"F10 N3 F7 R90 F11";

  let actions = parse_actions(&input_all)?;
  p1(&actions)?;
  p2(&actions)?;
  Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
  North,
  South,
  East,
  West,
}
impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Direction::North => write!(f, "North"),
      Direction::South => write!(f, "South"),
      Direction::East => write!(f, "East"),
      Direction::West => write!(f, "West"),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Action {
  GoNorth(u32),
  GoSouth(u32),
  GoEast(u32),
  GoWest(u32),
  GoForward(u32),
  TurnLeft(u32),
  TurnRight(u32),
}
impl std::fmt::Display for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Action::GoNorth(x) => write!(f, "GoNorth {}", x),
      Action::GoSouth(x) => write!(f, "GoSouth {}", x),
      Action::GoEast(x) => write!(f, "GoEast {}", x),
      Action::GoWest(x) => write!(f, "GoWest {}", x),
      Action::TurnLeft(x) => write!(f, "TurnLeft {}", x),
      Action::TurnRight(x) => write!(f, "TurnRight {}", x),
      Action::GoForward(x) => write!(f, "GoForward {}", x),
    }
  }
}

fn parse_actions(input_all: &str) -> anyhow::Result<Vec<Action>> {
  let mut v = Vec::<Action>::new();
  for l in input_all.split_whitespace() {
    let action = match l.chars().nth(0).unwrap() {
      'N' => Action::GoNorth(l[1..].parse::<u32>()?),
      'S' => Action::GoSouth(l[1..].parse::<u32>()?),
      'E' => Action::GoEast(l[1..].parse::<u32>()?),
      'W' => Action::GoWest(l[1..].parse::<u32>()?),
      'F' => Action::GoForward(l[1..].parse::<u32>()?),
      'L' => Action::TurnLeft(l[1..].parse::<u32>()?),
      'R' => Action::TurnRight(l[1..].parse::<u32>()?),
      _ => return Err(anyhow!("Bad input {}", l)),
    };
    v.push(action);
  }
  Ok(v)
}

fn turn(d: Direction, a: Action) -> anyhow::Result<Direction> {
  match a {
    Action::TurnLeft(0) => Ok(d),
    Action::TurnRight(0) => Ok(d),
    Action::TurnLeft(deg) => {
      if deg > 360 || deg % 90 > 0 {
        return Err(anyhow!("Bad input degree {}", deg));
      }
      let turned = match d {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
      };
      turn(turned, Action::TurnLeft(deg - 90))
    }
    Action::TurnRight(deg) => turn(d, Action::TurnLeft(360 - deg)),
    _ => Ok(d),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_turn() -> anyhow::Result<()> {
    const N: Direction = Direction::North;
    const S: Direction = Direction::South;
    const E: Direction = Direction::East;
    const W: Direction = Direction::West;

    assert_eq!(turn(N, Action::TurnLeft(0))?, N);
    assert_eq!(turn(S, Action::TurnLeft(0))?, S);
    assert_eq!(turn(E, Action::TurnLeft(0))?, E);
    assert_eq!(turn(W, Action::TurnLeft(0))?, W);

    assert_eq!(turn(N, Action::TurnLeft(90))?, W);
    assert_eq!(turn(W, Action::TurnLeft(90))?, S);
    assert_eq!(turn(S, Action::TurnLeft(90))?, E);
    assert_eq!(turn(E, Action::TurnLeft(90))?, N);

    assert_eq!(turn(N, Action::TurnRight(90))?, E);
    assert_eq!(turn(E, Action::TurnRight(90))?, S);
    assert_eq!(turn(S, Action::TurnRight(90))?, W);
    assert_eq!(turn(W, Action::TurnRight(90))?, N);

    assert_eq!(turn(N, Action::TurnLeft(180))?, S);
    assert_eq!(turn(W, Action::TurnLeft(180))?, E);
    assert_eq!(turn(S, Action::TurnLeft(180))?, N);
    assert_eq!(turn(E, Action::TurnLeft(180))?, W);

    assert_eq!(turn(N, Action::TurnRight(180))?, S);
    assert_eq!(turn(E, Action::TurnRight(180))?, W);
    assert_eq!(turn(S, Action::TurnRight(180))?, N);
    assert_eq!(turn(W, Action::TurnRight(180))?, E);

    assert_eq!(turn(N, Action::TurnLeft(270))?, E);
    assert_eq!(turn(E, Action::TurnLeft(270))?, S);
    assert_eq!(turn(S, Action::TurnLeft(270))?, W);
    assert_eq!(turn(W, Action::TurnLeft(270))?, N);

    assert_eq!(turn(N, Action::TurnRight(270))?, W);
    assert_eq!(turn(W, Action::TurnRight(270))?, S);
    assert_eq!(turn(S, Action::TurnRight(270))?, E);
    assert_eq!(turn(E, Action::TurnRight(270))?, N);

    assert_eq!(turn(E, Action::GoEast(1))?, E);
    assert_eq!(turn(E, Action::GoWest(1))?, E);
    assert_eq!(turn(E, Action::GoNorth(1))?, E);
    assert_eq!(turn(E, Action::GoSouth(1))?, E);
    assert_eq!(turn(E, Action::GoForward(1))?, E);

    assert!(turn(E, Action::TurnRight(51)).is_err());
    Ok(())
  }
}

fn move_delta(dir: Direction, action: Action) -> (i32, i32) {
  match action {
    Action::GoNorth(dist) => (0, dist as i32),
    Action::GoSouth(dist) => (0, -(dist as i32)),
    Action::GoEast(dist) => (dist as i32, 0),
    Action::GoWest(dist) => (-(dist as i32), 0),
    Action::GoForward(dist) => match dir {
      Direction::North => move_delta(dir, Action::GoNorth(dist)),
      Direction::South => move_delta(dir, Action::GoSouth(dist)),
      Direction::East => move_delta(dir, Action::GoEast(dist)),
      Direction::West => move_delta(dir, Action::GoWest(dist)),
    },
    Action::TurnLeft(_) | Action::TurnRight(_) => (0, 0),
  }
}

struct Step {
  dir: Direction,
  x: i32,
  y: i32,
}

fn perform_action(step: Step, action: Action) -> anyhow::Result<Step> {
  let dir = turn(step.dir, action)?;
  let (xd, yd) = move_delta(step.dir, action);
  Ok(Step {
    dir: dir,
    x: step.x + xd,
    y: step.y + yd,
  })
}

fn manhattan_distance(x: i32, y: i32) -> u32 {
  (x.abs() + y.abs()) as u32
}

fn p1(actions: &Vec<Action>) -> anyhow::Result<()> {
  let mut dir = Direction::East;
  let mut x = 0;
  let mut y = 0;
  for action in actions {
    let step = perform_action(
      Step {
        dir: dir,
        x: x,
        y: y,
      },
      *action,
    )?;
    dir = step.dir;
    x = step.x;
    y = step.y;
  }
  println!("Part 1 {}", manhattan_distance(x, y));
  Ok(())
}

struct Step2 {
  x: i32,
  y: i32,
  waypoint_dx: i32,
  waypoint_dy: i32,
}

fn turn_waypoint(x: i32, y: i32, a: Action) -> anyhow::Result<(i32, i32)> {
  match a {
    Action::TurnLeft(0) | Action::TurnRight(0) => Ok((x, y)),
    Action::TurnLeft(deg) => turn_waypoint(x, y, Action::TurnRight(360 - deg)),
    Action::TurnRight(deg) => {
      if deg > 360 || deg % 90 > 0 {
        return Err(anyhow!("Bad input degree {}", deg));
      }
      let (x, y) = (y, -x); // 90 degree rotation to the right.
      turn_waypoint(x, y, Action::TurnRight(deg - 90))
    }
    _ => Ok((x, y)),
  }
}

fn perform_action2(step: Step2, action: Action) -> anyhow::Result<Step2> {
  let (waypoint_dx, waypoint_dy) = turn_waypoint(step.waypoint_dx, step.waypoint_dy, action)?;
  match action {
    Action::GoForward(dist) => Ok(Step2 {
      x: step.x + waypoint_dx * dist as i32,
      y: step.y + waypoint_dy * dist as i32,
      waypoint_dx: waypoint_dx,
      waypoint_dy: waypoint_dy,
    }),
    _ => {
      let (xd, yd) = move_delta(/*unused*/ Direction::North, action);
      Ok(Step2 {
        x: step.x,
        y: step.y,
        waypoint_dx: waypoint_dx + xd,
        waypoint_dy: waypoint_dy + yd,
      })
    }
  }
}

fn p2(actions: &Vec<Action>) -> anyhow::Result<()> {
  let mut ship_x = 0;
  let mut ship_y = 0;
  // Waypoint starts North 1, East 10 compared to the ship.
  let mut waypoint_dx = 10;
  let mut waypoint_dy = 1;
  for action in actions {
    let step = perform_action2(
      Step2 {
        x: ship_x,
        y: ship_y,
        waypoint_dx: waypoint_dx,
        waypoint_dy: waypoint_dy,
      },
      *action,
    )?;
    ship_x = step.x;
    ship_y = step.y;
    waypoint_dx = step.waypoint_dx;
    waypoint_dy = step.waypoint_dy;
  }
  println!("Part 2 {}", manhattan_distance(ship_x, ship_y));
  Ok(())
}
