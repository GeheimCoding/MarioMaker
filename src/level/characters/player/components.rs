use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum State {
    #[default]
    Idle,
    IdleWithGrab,
    Running,
    RunningWithGrab,
    Jumping,
    JumpingWithGrab,
    Falling,
    FallingWithGrab,
    Gazing,
    GazingWithGrab,
    Grouching,
    GrouchingWithGrab,
    Kicking,
}

impl State {
    pub fn is_crouching(&self) -> bool {
        self == &Self::Grouching || self == &Self::GrouchingWithGrab
    }

    pub fn get_grabbed_variant(&self) -> Self {
        match self {
            Self::Idle => Self::IdleWithGrab,
            Self::Running => Self::RunningWithGrab,
            Self::Jumping => Self::JumpingWithGrab,
            Self::Falling => Self::FallingWithGrab,
            Self::Gazing => Self::GazingWithGrab,
            Self::Grouching => Self::GrouchingWithGrab,
            default => *default,
        }
    }

    pub fn get_variant_without_grab(&self) -> Self {
        match self {
            Self::IdleWithGrab => Self::Idle,
            Self::RunningWithGrab => Self::Running,
            Self::JumpingWithGrab => Self::Jumping,
            Self::FallingWithGrab => Self::Falling,
            Self::GazingWithGrab => Self::Gazing,
            Self::GrouchingWithGrab => Self::Grouching,
            default => *default,
        }
    }
}

#[derive(Component)]
pub struct KickTimer(pub Timer);
