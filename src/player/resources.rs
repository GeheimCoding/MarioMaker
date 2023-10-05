use crate::components::Animation;
use crate::player::components::State;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Mario,
}

#[derive(Resource)]
pub struct Textures(HashMap<Texture, Handle<TextureAtlas>>);

#[derive(Resource)]
pub struct Animations(HashMap<State, Animation>);

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let textures = HashMap::from([(
            Texture::Mario,
            get_handle(world, "textures/mario.png", Vec2::new(16.0, 24.0), 3, 1),
        )]);
        Self(textures)
    }
}

impl Default for Animations {
    fn default() -> Self {
        let animations = HashMap::from([
            (State::Idle, Animation::once(0)),
            (State::Walking, Animation::repeating(0.15, vec![0, 1], 1)),
            (State::Jumping, Animation::once(2)),
        ]);
        Self(animations)
    }
}

impl Textures {
    pub fn get(&self, texture: &Texture) -> Handle<TextureAtlas> {
        self.0
            .get(texture)
            .expect(&format!("Texture {:?} not found", texture))
            .clone()
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

fn get_handle(
    world: &mut World,
    path: &str,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
) -> Handle<TextureAtlas> {
    let texture_handle = world
        .get_resource::<AssetServer>()
        .expect("AssetServer not available")
        .load(path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, tile_size, columns, rows, None, None);

    world
        .get_resource_mut::<Assets<TextureAtlas>>()
        .expect("Assets not available")
        .add(texture_atlas)
}
