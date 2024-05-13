use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

pub const ICON_INDEX_SCROLL_MARKER: usize = 56;

#[derive(AssetCollection, Resource)]
pub struct GameSprites {
    #[asset(path = "sprites/background.png")]
    pub background: Handle<Image>,

    #[asset(path = "sprites/inventory_bg.png")]
    pub inventory_bg: Handle<Image>,

    #[asset(path = "sprites/inventory_scroll.png")]
    pub inventory_scroll: Handle<Image>,

    #[asset(path = "sprites/loot_scroll.png")]
    pub loot_scroll: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 8))]
    pub items_tile_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/items_tile_sheet.png")]
    pub items_tile_sheet: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GameFonts {
    #[asset(path = "fonts/minimal5x7.ttf")]
    pub font: Handle<Font>,
}
