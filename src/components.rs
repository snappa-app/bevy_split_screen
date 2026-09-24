use bevy::prelude::*;

/// Marks a camera for occupying one of the split screen viewports, ordered by `player_index`.
#[derive(Component, Reflect)]
pub struct SplitScreenCamera {
  pub player_index: u32,
}

impl SplitScreenCamera {
  pub fn new(player_index: u32) -> Self {
    Self { player_index }
  }
}
