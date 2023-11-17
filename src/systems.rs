use crate::components::{
    Animation, Cursor, Direction, Gravity, MainCamera, UiCamera, Velocity, UI_LAYER,
};
use crate::level::characters::components::Grabbed;
use crate::level::characters::player::movement::components::JumpTimer;
use crate::resources::{AppState, MousePosition};
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

pub fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<&mut Window>,
) {
    let mut window = query.single_mut();
    window.cursor.visible = false;

    let texture_handle = asset_server.load("textures/cursor.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::splat(64.0), 2, 1, None, None);

    commands.spawn((
        Cursor,
        RenderLayers::layer(UI_LAYER),
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            ..default()
        },
    ));
}

pub fn despawn_cursor(mut commands: Commands, query: Query<Entity, With<Cursor>>) {
    for cursor in query.iter() {
        commands.entity(cursor).despawn();
    }
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

pub fn move_cursor(
    window_query: Query<&Window, With<Window>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<UiCamera>>,
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
) {
    let offset = 24.0;
    let mut transform = cursor_query.single_mut();
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        transform.translation =
            Vec3::new(world_position.x + offset, world_position.y - offset, 0.0);
    }
}

pub fn update_cursor_sprite(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut TextureAtlasSprite, With<Cursor>>,
) {
    let mut cursor = query.single_mut();
    if mouse_button_input.pressed(MouseButton::Left) {
        cursor.index = 1;
    } else {
        cursor.index = 0;
    }
}

pub fn change_state(
    keyboard_input: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match state.get() {
        AppState::Editor => {
            if keyboard_input.just_pressed(KeyCode::L) {
                next_state.set(AppState::Level);
            }
        }
        AppState::Level => {
            if keyboard_input.just_pressed(KeyCode::E) {
                next_state.set(AppState::Editor);
            }
        }
    }
}
