use bevy::prelude::*;

/// A viewport's position and size, in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitScreenViewport {
  pub position: UVec2,
  pub size: UVec2,
}

/// Computes viewport rectangles for split screen cameras.
pub trait SplitScreenLayout {
  /// The largest camera count this layout knows how to arrange.
  const MAX_SPLIT_SCREEN_COUNT: u32;

  /// Returns `player_index`'s viewport within `window_size` given `total_players` active
  /// cameras.
  fn calculate_split_screen_layout(
    player_index: u32,
    total_players: u32,
    window_size: UVec2,
  ) -> SplitScreenViewport;
}

/// Grid layout supporting up to 12 players
pub struct DefaultSplitScreenLayout;

impl SplitScreenLayout for DefaultSplitScreenLayout {
  const MAX_SPLIT_SCREEN_COUNT: u32 = 12;

  fn calculate_split_screen_layout(
    player_index: u32,
    total_players: u32,
    window_size: UVec2,
  ) -> SplitScreenViewport {
    let (position, size) = match total_players {
      1 => (UVec2::ZERO, window_size),
      2 => {
        let size = UVec2::new(window_size.x, window_size.y / 2);
        let pos = UVec2::new(0, player_index * size.y);
        (pos, size)
      }
      3 => {
        if player_index == 0 {
          (UVec2::ZERO, UVec2::new(window_size.x, window_size.y / 2))
        } else {
          let size = UVec2::new(window_size.x / 2, window_size.y / 2);
          let pos = UVec2::new((player_index - 1) * size.x, window_size.y / 2);
          (pos, size)
        }
      }
      4 => {
        let size = window_size / 2;
        let pos = UVec2::new((player_index % 2) * size.x, (player_index / 2) * size.y);
        (pos, size)
      }
      5 => {
        if player_index < 2 {
          let size = UVec2::new(window_size.x / 2, window_size.y / 2);
          let pos = UVec2::new(player_index * size.x, 0);
          (pos, size)
        } else {
          let size = UVec2::new(window_size.x / 3, window_size.y / 2);
          let pos = UVec2::new((player_index - 2) * size.x, window_size.y / 2);
          (pos, size)
        }
      }
      6 => {
        let size = UVec2::new(window_size.x / 3, window_size.y / 2);
        let pos = UVec2::new((player_index % 3) * size.x, (player_index / 3) * size.y);
        (pos, size)
      }
      7 => {
        let row = if player_index < 2 {
          0
        } else if player_index < 4 {
          1
        } else {
          2
        };
        let col_in_row = if player_index < 2 {
          player_index
        } else if player_index < 4 {
          player_index - 2
        } else {
          player_index - 4
        };
        let cols_in_row = if row == 2 { 3 } else { 2 };

        let size = UVec2::new(window_size.x / cols_in_row, window_size.y / 3);
        let pos = UVec2::new(col_in_row * size.x, row * size.y);
        (pos, size)
      }
      8 => {
        let size = UVec2::new(window_size.x / 2, window_size.y / 4);
        let pos = UVec2::new((player_index % 2) * size.x, (player_index / 2) * size.y);
        (pos, size)
      }
      9 => {
        let size = window_size / 3;
        let pos = UVec2::new((player_index % 3) * size.x, (player_index / 3) * size.y);
        (pos, size)
      }
      10 => {
        // 4 rows: 2, 3, 2, 3
        let (row, col_in_row, cols_in_row) = if player_index < 2 {
          (0, player_index, 2) // Row 0: 2 across
        } else if player_index < 5 {
          (1, player_index - 2, 3) // Row 1: 3 across
        } else if player_index < 7 {
          (2, player_index - 5, 2) // Row 2: 2 across
        } else {
          (3, player_index - 7, 3) // Row 3: 3 across
        };

        let size = UVec2::new(window_size.x / cols_in_row, window_size.y / 4);
        let pos = UVec2::new(col_in_row * size.x, row * size.y);
        (pos, size)
      }
      11 => {
        // 4 rows: 2, 3, 3, 3
        let (row, col_in_row, cols_in_row) = if player_index < 2 {
          (0, player_index, 2) // Row 0: 2 across
        } else if player_index < 5 {
          (1, player_index - 2, 3) // Row 1: 3 across
        } else if player_index < 8 {
          (2, player_index - 5, 3) // Row 2: 3 across
        } else {
          (3, player_index - 8, 3) // Row 3: 3 across
        };

        let size = UVec2::new(window_size.x / cols_in_row, window_size.y / 4);
        let pos = UVec2::new(col_in_row * size.x, row * size.y);
        (pos, size)
      }
      12 => {
        // 4 rows: 3, 3, 3, 3 (simple 3x4 grid)
        let size = UVec2::new(window_size.x / 3, window_size.y / 4);
        let pos = UVec2::new((player_index % 3) * size.x, (player_index / 3) * size.y);
        (pos, size)
      }
      _ => {
        // More than 12 players - fallback to 3x4 grid layout
        let size = UVec2::new(window_size.x / 3, window_size.y / 4);
        let pos = UVec2::new((player_index % 3) * size.x, (player_index / 3) * size.y);
        (pos, size)
      }
    };
    SplitScreenViewport { position, size }
  }
}
