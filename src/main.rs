mod assets;
mod battle;
mod common;
mod enemy;
mod inventory;
mod items;
mod log;
mod music;
mod numoids;
mod player;
mod rng;
mod scene;
mod tooltip;
mod ui;

use assets::{custom_load_assets, GameAudio, GameFonts, GameSprites, TextUIMaterial};
use battle::BattlePlugin;
use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use enemy::EnemyPlugin;
use inventory::InventoryPlugin;
use items::ItemPlugin;
// use log::BattleLogPlugin;
use music::MusicPlugin;
use numoids::NumoidPlugin;
use player::PlayerPlugin;
use rng::RngPlugin;
use scene::ScenePlugin;
use tooltip::TooltipPlugin;
use ui::UIPlugin;

const GAME_WIDTH: f32 = 320.;
const GAME_HEIGHT: f32 = 180.;

const MONITOR_WIDTH: f32 = 1920.;
const MONITOR_HEIGHT: f32 = 1080.;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(MONITOR_WIDTH, MONITOR_HEIGHT)
                            .with_scale_factor_override(MONITOR_WIDTH / GAME_WIDTH),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RngPlugin)
        .add_plugins(UiMaterialPlugin::<TextUIMaterial>::default())
        .add_plugins(UIPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(MusicPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ItemPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(BattlePlugin)
        // .add_plugins(BattleLogPlugin)
        .add_plugins(TooltipPlugin)
        .add_plugins(NumoidPlugin)
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::InitGame)
                .load_collection::<GameSprites>()
                .load_collection::<GameFonts>()
                .load_collection::<GameAudio>(),
        )
        .add_systems(OnEnter(AppState::LoadingAssets), custom_load_assets)
        .add_systems(OnEnter(AppState::InitGame), setup_scene)
        .add_systems(OnEnter(AppState::GameStart), reset_battle_wins)
        // .add_systems(
        //     Update,
        //     (
        //         restart_game.run_if(in_state(AppState::GameOver)),
        //         start_game.run_if(in_state(AppState::GameStart)),
        //         start_battle.run_if(in_state(AppState::OrganizeInventory)),
        //     ),
        // )
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    LoadingAssets,
    InitGame,
    GameStart,
    OrganizeInventory,
    Battling,
    GameOver,
}

#[derive(Resource, Default)]
pub struct BattleWins(pub usize);

fn setup_scene(mut commands: Commands, mut next_app_state: ResMut<NextState<AppState>>) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });

    next_app_state.set(AppState::GameStart);
}

fn reset_battle_wins(mut commands: Commands) {
    commands.insert_resource(BattleWins::default());
}

// fn restart_game(
//     mut next_app_state: ResMut<NextState<AppState>>,
//     key_codes: Res<ButtonInput<KeyCode>>,
// ) {
//     if key_codes.just_pressed(KeyCode::Space) {
//         next_app_state.set(AppState::GameStart);
//     }
// }
//
// fn start_game(
//     mut next_app_state: ResMut<NextState<AppState>>,
//     key_codes: Res<ButtonInput<KeyCode>>,
// ) {
//     if key_codes.just_pressed(KeyCode::Space) {
//         next_app_state.set(AppState::OrganizeInventory);
//     }
// }
//
// fn start_battle(
//     mut next_app_state: ResMut<NextState<AppState>>,
//     key_codes: Res<ButtonInput<KeyCode>>,
// ) {
//     if key_codes.just_pressed(KeyCode::Space) {
//         next_app_state.set(AppState::Battling);
//     }
// }
