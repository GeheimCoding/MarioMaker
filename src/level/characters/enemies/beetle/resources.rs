use crate::components::Animation;
use crate::content_manager::TextureResource;
use crate::level::characters::enemies::beetle::components::State;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Beetle,
}

impl TextureResource for Texture {}

#[derive(Resource)]
pub struct Animations(HashMap<State, Animation>);

impl Default for Animations {
    fn default() -> Self {
        let animations = HashMap::from([
            (State::IdleAlive, Animation::once(1)),
            (State::IdleDead, Animation::once(2)),
            (State::Walking, Animation::repeating(0.2, vec![0, 1], 0)),
            (
                State::Rolling,
                Animation::repeating(0.1, vec![2, 3, 4, 5], 3),
            ),
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
