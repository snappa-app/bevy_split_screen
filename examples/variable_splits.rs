//! Split screen demo: press 1-9/0 or +/- to change the number of splits.
//! Each split is a camera at a different angle around a spinning green cube.
//!
//! Usage:
//!   cargo run -p bevy_split_screen --example variable_splits

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_split_screen::prelude::*;

const MIN_SPLIT_COUNT: u32 = 1;
const CAMERA_DISTANCE: f32 = 8.0;
const CAMERA_HEIGHT: f32 = 4.0;
const CUBE_SIZE: f32 = 2.0;
const CUBE_SPIN_SPEED: f32 = 0.6;
const UI_BORDER_WIDTH: f32 = 2.0;
const UI_CORNER_LABEL_OFFSET: f32 = 8.0;
const UI_CORNER_LABEL_FONT_SIZE: f32 = 24.0;

const DIGIT_KEYS: [(KeyCode, u32); 10] = [
  (KeyCode::Digit1, 1),
  (KeyCode::Digit2, 2),
  (KeyCode::Digit3, 3),
  (KeyCode::Digit4, 4),
  (KeyCode::Digit5, 5),
  (KeyCode::Digit6, 6),
  (KeyCode::Digit7, 7),
  (KeyCode::Digit8, 8),
  (KeyCode::Digit9, 9),
  (KeyCode::Digit0, 10),
];

#[derive(Component)]
struct SpinningCube;

#[derive(Component, Default, Clone)]
struct SplitScreenOverlayUi;

#[derive(Resource)]
struct SplitCount(u32);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(SplitScreenPlugin)
    .insert_resource(SplitCount(MIN_SPLIT_COUNT))
    .add_systems(Startup, setup)
    .add_systems(
      Update,
      (
        spin_cube_system,
        update_split_count_system,
        sync_cameras_system,
      )
        .chain()
        // Ensures we can spawn and despawn the cameras before the windows update to account for it
        // and prevents hitching
        .before(SplitScreenSystemSet),
    )
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Color::srgb(0.1, 0.8, 0.2),
      ..default()
    })),
    Transform::default(),
    SpinningCube,
  ));

  commands.spawn((
    DirectionalLight {
      illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
      ..default()
    },
    Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-1.0, -1.0, -0.5), Vec3::Y),
  ));

  commands.insert_resource(GlobalAmbientLight {
    color: Color::WHITE,
    brightness: 300.0,
    affects_lightmapped_meshes: false,
  });
}

fn spin_cube_system(time: Res<Time>, mut cube_query: Query<&mut Transform, With<SpinningCube>>) {
  for mut transform in cube_query.iter_mut() {
    transform.rotate_y(CUBE_SPIN_SPEED * time.delta_secs());
  }
}

fn update_split_count_system(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut split_count: ResMut<SplitCount>,
) {
  for (key, count) in DIGIT_KEYS {
    if keyboard.just_pressed(key) {
      split_count.0 = count;
    }
  }

  let increment_pressed =
    keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd);
  if increment_pressed {
    split_count.0 = (split_count.0 + 1).min(DefaultSplitScreenLayout::MAX_SPLIT_SCREEN_COUNT);
  }

  let decrement_pressed =
    keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract);
  if decrement_pressed {
    split_count.0 = (split_count.0 - 1).max(MIN_SPLIT_COUNT);
  }
}

fn sync_cameras_system(
  mut commands: Commands,
  split_count: Res<SplitCount>,
  camera_query: Query<Entity, With<SplitScreenCamera>>,
  overlay_query: Query<Entity, With<SplitScreenOverlayUi>>,
) {
  if !split_count.is_changed() {
    return;
  }
  for entity in camera_query.iter() {
    commands.entity(entity).despawn();
  }
  for entity in overlay_query.iter() {
    commands.entity(entity).despawn();
  }
  for index in 0..split_count.0 {
    spawn_camera(&mut commands, index, split_count.0);
  }
}

fn spawn_camera(commands: &mut Commands, index: u32, total: u32) {
  let angle = index as f32 / total as f32 * TAU;
  let position = Vec3::new(
    angle.cos() * CAMERA_DISTANCE,
    CAMERA_HEIGHT,
    angle.sin() * CAMERA_DISTANCE,
  );

  let camera = commands
    .spawn((
      SplitScreenCamera::new(index),
      Camera3d::default(),
      Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
    ))
    .id();

  let label = index.to_string();
  commands
    .spawn_scene(bsn! {
      SplitScreenOverlayUi
      Node {
        width: percent(100.0),
        height: percent(100.0),
        border: UiRect::all(px(UI_BORDER_WIDTH)),
      }
      BorderColor::all(Color::WHITE)
      Children [
        Node {
          position_type: PositionType::Absolute,
          left: px(UI_CORNER_LABEL_OFFSET),
          top: px(UI_CORNER_LABEL_OFFSET),
        }
        Text(label)
        TextFont { font_size: FontSize::Px(UI_CORNER_LABEL_FONT_SIZE) }
        TextColor(Color::WHITE)
      ]
    })
    .insert(UiTargetCamera(camera));
}
