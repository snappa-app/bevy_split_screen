use bevy::{camera::Viewport, prelude::*, window::WindowResized};

use crate::{components::SplitScreenCamera, layout::SplitScreenLayout};

/// Recomputes and assigns each `SplitScreenCamera`'s viewport when the window resizes or a
/// camera is added.
pub(crate) fn set_camera_viewport_system<L: SplitScreenLayout>(
  window_query: Query<&Window>,
  mut window_resized_reader: MessageReader<WindowResized>,
  added_camera_query: Query<(), Added<SplitScreenCamera>>,
  mut query: Query<(&SplitScreenCamera, &mut Camera)>,
) {
  for window_resized in window_resized_reader.read() {
    let Ok(window) = window_query.get(window_resized.window) else {
      continue;
    };
    update_viewport_for_window::<L>(window, &mut query);
  }
  // Added cameras can come later in the game even if a resize didn't happen
  for _ in added_camera_query.iter() {
    for window in window_query.iter() {
      update_viewport_for_window::<L>(window, &mut query);
    }
  }
}

fn update_viewport_for_window<L: SplitScreenLayout>(
  window: &Window,
  camera_query: &mut Query<(&SplitScreenCamera, &mut Camera)>,
) {
  let Some(max_player_index) = camera_query
    .iter()
    .map(|(camera, _)| camera.player_index)
    .max()
  else {
    // No players yet
    return;
  };
  for (camera_assignment, mut camera) in camera_query.iter_mut() {
    let viewport = L::calculate_split_screen_layout(
      camera_assignment.player_index,
      max_player_index + 1,
      window.physical_size(),
    );
    camera.viewport = Some(Viewport {
      physical_position: viewport.position,
      physical_size: viewport.size,
      ..default()
    });
  }
}
