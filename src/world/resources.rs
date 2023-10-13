use crate::content_manager::TextureResource;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Block,
}

impl TextureResource for Texture {}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct Tiles(pub HashMap<(isize, isize), Entity>);
