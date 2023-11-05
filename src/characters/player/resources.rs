use crate::characters::player::components::State;
use crate::components::Animation;
use crate::content_manager::TextureResource;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Mario,
}

impl TextureResource for Texture {}

#[derive(Resource)]
pub struct Animations(HashMap<State, Animation>);

impl Default for Animations {
    fn default() -> Self {
        let animations = HashMap::from([
            (State::Idle, Animation::once(0)),
            (State::Running, Animation::repeating(0.15, vec![0, 3], 1)),
            (State::Jumping, Animation::once(7)),
            (State::Falling, Animation::once(15)),
            (State::Gazing, Animation::once(1)),
            (State::Grouching, Animation::once(2)),
        ]);
        Self(animations)
    }
}

impl Animations {
    pub fn get(&self, state: &State) -> Animation {
        self.0
            .get(state)
            .expect(&format!("State {:?} not found", state))
            .clone()
    }
}
