use crate::content_manager::TextureResource;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    Block,
}

impl TextureResource for Texture {}
