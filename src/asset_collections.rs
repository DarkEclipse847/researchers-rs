use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct DummyIdleSpriteSheet{
    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 4, rows = 1))]
    pub dummy_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/mini-dummy-Sheet.png")]
    pub dummy: Handle<Image>,
}