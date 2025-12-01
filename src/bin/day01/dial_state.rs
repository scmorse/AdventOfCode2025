#[derive(Debug, Default)]
pub struct DialState {
  pub current_dial_position: u32,
  pub zero_position_count: u32,
}

impl DialState {
  pub fn new() -> Self {
    Self {
      current_dial_position: 50,
      ..Default::default()
    }
  }
}
