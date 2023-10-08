use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

#[derive(Component, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
    pub max: Vec2,
}

impl Velocity {
    pub fn with_max(max: Vec2) -> Self {
        Self {
            value: Vec2::ZERO,
            max,
        }
    }
}

pub const MIN_ANIMATION_DURATION: f32 = 0.1;

#[derive(Clone, Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub frame_index: usize,
}

impl Animation {
    pub fn once(frame: usize) -> Self {
        Self {
            timer: Timer::from_seconds(MIN_ANIMATION_DURATION, TimerMode::Once),
            frames: vec![frame],
            frame_index: 0,
        }
    }

    pub fn repeating(duration_seconds: f32, frames: Vec<usize>, start_frame_index: usize) -> Self {
        Self {
            timer: Timer::from_seconds(duration_seconds, TimerMode::Repeating),
            frames,
            frame_index: start_frame_index,
        }
    }
}

#[derive(Component)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

impl Collider {
    pub fn with_size(size: Vec2) -> Self {
        Self {
            size,
            offset: Vec2::ZERO,
        }
    }

    pub fn half_size(&self) -> Vec2 {
        self.size / 2.0
    }

    pub fn position_response(&self, rect: &Rect) -> Rect {
        Rect {
            min: rect.min - self.offset - self.half_size(),
            max: rect.max - self.offset + self.half_size(),
        }
    }

    pub fn get_rect(&self, transform: &Transform) -> Rect {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        Rect {
            min: position + self.offset - self.half_size(),
            max: position + self.offset + self.half_size(),
        }
    }
}
