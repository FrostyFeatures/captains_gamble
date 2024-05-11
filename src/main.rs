mod assets;
mod battle;
mod dice;
mod inventory;
mod items;

use assets::GameSprites;
use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use inventory::InventoryPlugin;
use rand::rngs::ThreadRng;

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
        .add_plugins(InventoryPlugin)
        .add_loading_state(
            LoadingState::new(AppState::LoadingAssets)
                .continue_to_state(AppState::InitGame)
                .load_collection::<GameSprites>(),
        )
        .add_systems(PreStartup, init_rng)
        .add_systems(OnEnter(AppState::InitGame), setup_scene)
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    LoadingAssets,
    InitGame,
    GameStart,
    PlunderBooty,
    OrganizeInventory,
    Battling,
    GameOver,
}

pub struct Rng(ThreadRng);

fn init_rng(world: &mut World) {
    world.insert_non_send_resource(Rng(rand::thread_rng()));
}

fn setup_scene(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });

    commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Global(i32::MIN),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(game_sprites.background.clone()),
                ..default()
            });
        });

    next_app_state.set(AppState::GameStart);
}
