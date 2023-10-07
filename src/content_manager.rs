use bevy::prelude::*;
use bevy::utils::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait TextureResource: Debug + Eq + Hash + PartialEq + Send + Sync + 'static {}

pub struct TextureData<Texture: TextureResource> {
    pub texture: Texture,
    pub path: String,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
}

#[derive(Resource)]
pub struct Textures<Texture: TextureResource>(HashMap<Texture, Handle<TextureAtlas>>);

impl<Texture: TextureResource> Textures<Texture> {
    pub fn init(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        texture_data: Vec<TextureData<Texture>>,
    ) {
        let mut textures = HashMap::new();
        for data in texture_data {
            let texture_handle = asset_server.load(data.path);
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                data.tile_size,
                data.columns,
                data.rows,
                None,
                None,
            );
            let texture_atlas = texture_atlases.add(texture_atlas);
            textures.insert(data.texture, texture_atlas);
        }
        commands.insert_resource(Textures(textures));
    }

    pub fn get(&self, texture: &Texture) -> Handle<TextureAtlas> {
        self.0
            .get(texture)
            .expect(&format!("Texture {:?} not found", texture))
            .clone()
    }
}
