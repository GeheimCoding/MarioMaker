use crate::world::components::TILE_SIZE;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Block,
}

#[derive(Resource)]
pub struct Textures(HashMap<Texture, Handle<TextureAtlas>>);

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let textures = HashMap::from([(
            Texture::Block,
            get_handle(world, "textures/block.png", Vec2::splat(TILE_SIZE), 1, 1),
        )]);
        Self(textures)
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
