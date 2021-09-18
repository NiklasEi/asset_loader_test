use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

fn main() {
    println!("Hello, world!");
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
    #[asset(texture_atlas(cell_width = 256., cell_height = 256., columns = 1, rows = 1))]
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy_atlas: Handle<TextureAtlas>,
}
