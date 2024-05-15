use bevy::prelude::*;

use crate::{
    assets::{GameFonts, GameSprites},
    common::Hp,
    ui::{BottomLeftUI, HealthBarUI, HealthBarUIText},
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

#[derive(Component, Default, Clone, Copy)]
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
    game_fonts: Res<GameFonts>,
    player_stats_ui_q: Query<Entity, With<BottomLeftUI>>,
    player_hp_q: Query<&Hp, With<Player>>,
) {
    commands
        .entity(player_stats_ui_q.single())
        .with_children(|mut parent| {
            HealthBarUI::spawn(
                &mut parent,
                &game_sprites,
                &game_fonts,
                &player_hp_q.single(),
                Player,
            );
        });
}

fn update_player_ui(
    mut health_bar_ui: Query<&mut TextureAtlas, (With<Player>, With<HealthBarUI>)>,
    mut health_bar_ui_text: Query<&mut Text, (With<Player>, With<HealthBarUIText>)>,
    player_hp_q: Query<&Hp, (With<Player>, Changed<Hp>)>,
) {
    if let Ok(hp) = player_hp_q.get_single() {
        health_bar_ui.single_mut().index = hp.health_bar_index();
        health_bar_ui_text
            .single_mut()
            .sections
            .get_mut(0)
            .unwrap()
            .value = format!("{hp}");
    }
}

fn destroy_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q.iter() {
        commands.entity(player).despawn_recursive();
    }
}
