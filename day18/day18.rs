#[macro_use]
extern crate anyhow;

enum CalcOp {
  Plus,
  Mult,
}

struct Calc {
  val_stack: Vec<i64>,
  op_stack: Vec<CalcOp>,
}
impl Calc {
  fn new() -> Self {
    Self {
      val_stack: vec![0],
      op_stack: vec![CalcOp::Plus],
    }
  }

  fn apply_number(&mut self, num: i64) {
    self.val_stack[0] = match self.op_stack[0] {
      CalcOp::Plus => self.val_stack[0] + num,
      CalcOp::Mult => self.val_stack[0] * num,
    }
  }
  fn apply_op(&mut self, op: CalcOp) {
    self.op_stack[0] = op;
  }
  fn apply_open_parens(&mut self) {
    assert_eq!(self.op_stack.len(), self.val_stack.len());
    self.val_stack.insert(0, 0);
    self.op_stack.insert(0, CalcOp::Plus);
  }
  fn apply_close_parens(&mut self) {
    assert_eq!(self.op_stack.len(), self.val_stack.len());
    assert!(self.op_stack.len() > 1);
    let paren_value = self.val_stack.remove(0);
    self.op_stack.remove(0);
    self.apply_number(paren_value);
  }

  fn compute_string(s: &str) -> anyhow::Result<i64> {
    let mut calc = Self::new();
    assert_eq!(calc.val_stack.len(), 1);
    for c in s.chars() {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          calc.apply_number(c.to_digit(10).unwrap() as i64)
        }
        '+' => calc.apply_op(CalcOp::Plus),
        '*' => calc.apply_op(CalcOp::Mult),
        '(' => calc.apply_open_parens(),
        ')' => calc.apply_close_parens(),
        ' ' => (),
        _ => return Err(anyhow!("bad input")),
      }
    }
    assert_eq!(calc.val_stack.len(), 1);
    Ok(calc.val_stack[0])
  }
}

fn p1(input_all: &str) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let mut sum = 0;
  for line in lines {
    sum += Calc::compute_string(line)?;
  }
  println!("Part 1 {}", sum);
  Ok(())
}

struct Calc2 {
  val_stack: Vec<i64>,
  op_stack: Vec<CalcOp>,
  mul_stack: Vec<i64>,
}
impl Calc2 {
  fn new() -> Self {
    Self {
      val_stack: vec![0],
      op_stack: vec![CalcOp::Plus],
      mul_stack: vec![1],
    }
  }

  fn apply_number(&mut self, num: i64) {
    self.val_stack[0] = match self.op_stack[0] {
      CalcOp::Plus => {
        self.val_stack[0] + num
      },
      CalcOp::Mult => {
        // Accumulate the lhs of the mult.
        self.mul_stack[0] *= self.val_stack[0];
        num
      },
    };
  }
  fn apply_op(&mut self, op: CalcOp) {
    match op {
      CalcOp::Plus => (),
      CalcOp::Mult => {
        // Apply the mult accumulator to the lhs.
        self.val_stack[0] *= self.mul_stack[0];
        self.mul_stack[0] = 1;
      },
    }
    self.op_stack[0] = op;
  }
  fn apply_open_parens(&mut self) {
    assert_eq!(self.op_stack.len(), self.val_stack.len());
    self.val_stack.insert(0, 0);
    self.op_stack.insert(0, CalcOp::Plus);
    self.mul_stack.insert(0, 1);
  }
  fn apply_close_parens(&mut self) {
    assert_eq!(self.op_stack.len(), self.val_stack.len());
    assert!(self.op_stack.len() > 1);
    // Apply the mult accumulator to the lhs.
    let paren_value = self.val_stack.remove(0) * self.mul_stack[0];
    self.op_stack.remove(0);
    self.mul_stack.remove(0);
    self.apply_number(paren_value);
  }

  fn compute_string(s: &str) -> anyhow::Result<i64> {
    let mut calc = Self::new();
    assert_eq!(calc.val_stack.len(), 1);
    for c in s.chars() {
      match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          calc.apply_number(c.to_digit(10).unwrap() as i64)
        }
        '+' => calc.apply_op(CalcOp::Plus),
        '*' => calc.apply_op(CalcOp::Mult),
        '(' => calc.apply_open_parens(),
        ')' => calc.apply_close_parens(),
        ' ' => (),
        _ => return Err(anyhow!("bad input")),
      }
    }
    assert_eq!(calc.val_stack.len(), 1);
    Ok(calc.val_stack[0] * calc.mul_stack[0])
  }
}

fn p2(input_all: &str) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let mut sum = 0;
  for line in lines {
    sum += Calc2::compute_string(line)?;
  }
  println!("Part 2 {}", sum);  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day18/input.txt")?;
  p1(&input_all)?;
  p2(&input_all)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_p1() -> anyhow::Result<()> {
    let cases = [
      ("1 + 2 * 3", 9),
      ("2 * 3 + (4 * 5)", 26),
      ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
      ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
      ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];
  
    for case in &cases {
      assert_eq!(case.1, Calc::compute_string(case.0)?);
    }
    Ok(())
  }

  #[test]
  fn test_p2() -> anyhow::Result<()> {
    let cases = [
      ("3 * 2 + 1", 9),
      ("1 + (2 * 3) + (4 * (5 + 6))", 51),
      ("2 * 3 + (4 * 5)", 46),
      ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
      ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
      ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];
  
    for case in &cases {
      assert_eq!(case.1, Calc2::compute_string(case.0)?);
    }
    Ok(())
  }
}
