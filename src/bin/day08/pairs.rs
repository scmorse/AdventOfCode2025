pub trait Pairs<T> {
  fn pairs(&self) -> Vec<(T, T)>;
}

impl<T: Copy> Pairs<T> for [T] {
  fn pairs(&self) -> Vec<(T, T)> {
    let mut result = Vec::with_capacity(self.len() * (self.len() - 1) / 2);
    for i in 0..self.len() {
      for j in (i + 1)..self.len() {
        result.push((self[i], self[j]));
      }
    }
    result
  }
}
