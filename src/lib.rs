//! Bevy split screen camera viewports for local multiplayer. Adding `SplitScreenCamera` to your
//! cameras assigns them to a viewport sized and positioned by the `SplitScreenLayout` trait.
//! Automatically updates when the window resizes or new cameras are added.

use std::marker::PhantomData;

use bevy::prelude::*;

pub mod prelude {
  pub use crate::components::SplitScreenCamera;
  pub use crate::layout::*;
  pub use crate::{CustomSplitScreenPlugin, SplitScreenPlugin, SplitScreenSystemSet};
}

mod components;
mod layout;
mod viewport_systems;

use layout::{DefaultSplitScreenLayout, SplitScreenLayout};

/// System set for assigning viewports to split screen cameras. Use this to order your systems
/// before the new viewport layout is computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct SplitScreenSystemSet;

/// Split screen plugin using the default horizontal layout
pub struct SplitScreenPlugin;

impl Plugin for SplitScreenPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(CustomSplitScreenPlugin::<DefaultSplitScreenLayout>::default());
  }
}

/// Split screen plugin generic over a `SplitScreenLayout`, for games that want their own viewport
/// layout instead of the default grid.
pub struct CustomSplitScreenPlugin<L = DefaultSplitScreenLayout> {
  _layout: PhantomData<L>,
}

impl<L> Default for CustomSplitScreenPlugin<L> {
  fn default() -> Self {
    Self {
      _layout: PhantomData,
    }
  }
}

impl<L> Plugin for CustomSplitScreenPlugin<L>
where
  L: SplitScreenLayout + Send + Sync + 'static,
{
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      viewport_systems::set_camera_viewport_system::<L>.in_set(SplitScreenSystemSet),
    );
  }
}
