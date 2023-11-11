use crate::ui::components::{LevelTimer, LevelTimerText};
use bevy::prelude::*;

pub fn spawn_level_timer(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let time = 500.0;
    commands.spawn((
        LevelTimerText,
        TextBundle::from_section(
            (time as u32).to_string(),
            TextStyle {
                font: asset_server.load("fonts/super-mario-maker-2/super-mario-maker-2.ttf"),
                font_size: 64.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(5.0),
            ..default()
        }),
    ));
    commands.spawn(LevelTimer(Timer::from_seconds(time + 1.0, TimerMode::Once)));
}

pub fn update_level_timer(
    time: Res<Time>,
    mut query: Query<&mut LevelTimer>,
    mut text_query: Query<&mut Text, With<LevelTimerText>>,
) {
    let mut level_timer = query.single_mut();
    let mut text = text_query.single_mut();
    if level_timer.0.tick(time.delta()).just_finished() {
        error!("time out");
    }
    text.sections[0].value = format!("{:03}", level_timer.0.remaining_secs() as i32);
}
