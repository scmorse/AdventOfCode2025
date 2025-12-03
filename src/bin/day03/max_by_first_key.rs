pub trait MaxByFirstKey {
  fn max_by_first_key(&self, start: usize, end: usize) -> usize;
}

impl MaxByFirstKey for [u32] {
  fn max_by_first_key(&self, start: usize, end: usize) -> usize {
    (start..end).rev().max_by_key(|&i| self[i]).unwrap()
  }
}
