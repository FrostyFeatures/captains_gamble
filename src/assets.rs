use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_asset_loader::asset_collection::AssetCollection;

pub const ICON_INDEX_SCROLL_MARKER: usize = 56;

#[derive(Resource)]
pub struct GameMaterials {
    pub text_bg: Handle<TextUIMaterial>,
}

#[derive(AssetCollection, Resource)]
pub struct GameSprites {
    #[asset(path = "sprites/background.png")]
    pub background: Handle<Image>,

    #[asset(path = "sprites/text_bg.png")]
    pub text_bg: Handle<Image>,

    #[asset(path = "sprites/inventory_bg.png")]
    pub inventory_bg: Handle<Image>,

    #[asset(path = "sprites/floor.png")]
    pub floor: Handle<Image>,

    #[asset(path = "sprites/inventory_scroll.png")]
    pub inventory_scroll: Handle<Image>,

    #[asset(path = "sprites/loot_scroll.png")]
    pub loot_scroll: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 8, rows = 8))]
    pub items_tile_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/items_tile_sheet.png")]
    pub items_tile_sheet: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 67., tile_size_y = 9., columns = 1, rows = 60))]
    pub health_bar_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/health_bar.png")]
    pub health_bar_sheet: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    pub pirate_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/pirate.png")]
    pub pirate_sheet: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    pub skeleton_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/skeleton_pirate.png")]
    pub skeleton_sheet: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GameFonts {
    #[asset(path = "fonts/minimal5x7.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAudio {
    #[asset(path = "audio/queen-annex27s-revenge-pirate-shanty-piano-solo-blackbeard-194624.mp3")]
    pub music: Handle<AudioSource>,
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct TextUIMaterial {
    #[uniform(0)]
    texture_resolution: Vec4,

    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
}

impl UiMaterial for TextUIMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/text_ui_material.wgsl".into()
    }
}

pub fn custom_load_assets(
    mut commands: Commands,
    mut text_ui_materials: ResMut<Assets<TextUIMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let text_bg_handle = asset_server.load("sprites/text_bg.png");
    let texture_dimentions = (16., 16., 0., 0.).into();

    commands.insert_resource(GameMaterials {
        text_bg: text_ui_materials.add(TextUIMaterial {
            texture: Some(text_bg_handle),
            texture_resolution: texture_dimentions,
        }),
    });
}
