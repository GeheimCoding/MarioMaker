use crate::characters::components::Grabbed;
use crate::characters::player::movement::components::JumpTimer;
use crate::components::{Animation, Direction, Gravity, MainCamera, UiCamera, Velocity, UI_LAYER};
use crate::resources::MousePosition;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        UiCameraConfig { show_ui: false },
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.25,
                ..Camera2dBundle::default().projection
            },
            ..default()
        },
    ));
    commands.spawn((
        UiCamera,
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(UI_LAYER),
    ));
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite, Option<&Direction>)>,
) {
    for (mut animation, mut sprite, direction) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            animation.frame_index = (animation.frame_index + 1) % animation.frames.len();
            sprite.index = animation.frames[animation.frame_index];
        }
        if let Some(direction) = direction {
            sprite.flip_x = direction == &Direction::Left;
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Gravity, Option<&JumpTimer>), Without<Grabbed>>,
) {
    for (mut velocity, gravity, jump_timer) in query.iter_mut() {
        if jump_timer.is_none() || jump_timer.unwrap().0.finished() {
            velocity.value.y = (velocity.value.y - gravity.0 * time.delta_seconds())
                .clamp(-velocity.max.y, velocity.value.y);
        }
    }
}

// https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn update_mouse_position(
    mut mouse_position: ResMut<MousePosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.0 = world_position;
    }
}
