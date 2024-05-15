use bevy::prelude::*;

use crate::{
    assets::GameSprites,
    common::Hp,
    ui::{BottomLeftUI, HealthBarUI},
    AppState,
};

const STARTING_PLAYER_HP: i32 = 10;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::GameStart),
            (setup_player, spawn_player_stats_ui).chain(),
        )
        .add_systems(OnExit(AppState::GameOver), destroy_player)
        .add_systems(
            Update,
            update_player_ui.run_if(any_with_component::<HealthBarUI>),
        );
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    hp: Hp,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            hp: Hp::new(STARTING_PLAYER_HP),
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn spawn_player_stats_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    player_stats_ui_q: Query<Entity, With<BottomLeftUI>>,
    player_hp_q: Query<&Hp, With<Player>>,
) {
    let health_bar = commands
        .spawn((
            HealthBarUI,
            AtlasImageBundle {
                image: UiImage::new(game_sprites.health_bar_sheet.clone()),
                texture_atlas: TextureAtlas {
                    layout: game_sprites.health_bar_layout.clone(),
                    index: player_hp_q.single().health_bar_index(),
                },
                ..default()
            },
        ))
        .id();

    commands
        .entity(player_stats_ui_q.single())
        .add_child(health_bar);
}

fn update_player_ui(
    mut health_bar_ui: Query<&mut TextureAtlas, With<HealthBarUI>>,
    player_hp_q: Query<&Hp, (With<Player>, Changed<Hp>)>,
) {
    if let Ok(hp) = player_hp_q.get_single() {
        health_bar_ui.single_mut().index = hp.health_bar_index()
    }
}

fn destroy_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q.iter() {
        commands.entity(player).despawn_recursive();
    }
}
