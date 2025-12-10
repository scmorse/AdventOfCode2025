pub type Point = (i64, i64);

#[derive(Debug, Eq, PartialEq)]
pub struct BoxWithArea(pub Point, pub Point);

impl BoxWithArea {
  pub fn area(&self) -> i64 {
    let (ax, ay) = self.0;
    let (bx, by) = self.1;
    ((ax - bx).abs() + 1) * ((ay - by).abs() + 1)
  }
}

impl Ord for BoxWithArea {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.area().cmp(&other.area())
  }
}

impl PartialOrd for BoxWithArea {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
