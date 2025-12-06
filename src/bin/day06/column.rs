#[derive(Debug, Clone)]
pub enum Column {
  Value(u64),
  ValueAndOp(u64, Op),
  Reset,
}

impl Column {
  pub fn value(&self) -> Option<u64> {
    match self {
      Column::Value(v) => Some(*v),
      Column::ValueAndOp(v, _) => Some(*v),
      Column::Reset => None,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
  Add,
  Multiply,
}

impl Op {
  pub fn from_char(c: char) -> Option<Self> {
    match c {
      '+' => Some(Op::Add),
      '*' => Some(Op::Multiply),
      _ => None,
    }
  }

  pub fn apply(&self, a: u64, b: u64) -> u64 {
    match self {
      Op::Add => a + b,
      Op::Multiply => a * b,
    }
  }
}
