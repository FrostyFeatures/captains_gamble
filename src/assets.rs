use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GameSprites {
    #[asset(path = "sprites/background.png")]
    pub background: Handle<Image>,

    #[asset(path = "sprites/inventory_bg.png")]
    pub inventory_bg: Handle<Image>,

    #[asset(path = "sprites/inventory_cloth.png")]
    pub inventory_cloth: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 8))]
    pub items_tile_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/items_tile_sheet.png")]
    pub items_tile_sheet: Handle<Image>,
}
