use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use std::fmt::Debug;

pub struct PlayerPlugin;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    speed: f32,
    direction: Direction,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (
                horizontal_player_movement.pipe(error_handler),
                animate_player.pipe(error_handler),
            ),
        );
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/mario.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 24.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Player {
            speed: 80.0,
            direction: Direction::Right,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

fn horizontal_player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) -> Result<(), QuerySingleError> {
    let (mut transform, mut player) = query.get_single_mut()?;

    let direction = get_direction(keyboard_input);
    if direction.x < 0.0 {
        player.direction = Direction::Left;
    } else if direction.x > 0.0 {
        player.direction = Direction::Right;
    }
    transform.translation += direction * player.speed * time.delta_seconds();

    Ok(())
}

fn get_direction(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec2::ZERO;
    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.0;
    }
    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
        direction.x += 1.0;
    }
    Vec3::new(direction.x, direction.y, 0.0)
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Player)>,
) -> Result<(), QuerySingleError> {
    let (mut timer, mut sprite, player) = query.get_single_mut()?;

    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = (sprite.index + 1) % 2;
    }
    sprite.flip_x = player.direction == Direction::Left;

    Ok(())
}

fn error_handler<E: Debug>(In(result): In<Result<(), E>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}
