use bevy::prelude::*;

use crate::{common::Hp, AppState};

const STARTING_PLAYER_HP: i32 = 10;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameStart), setup_player)
            .add_systems(OnExit(AppState::GameOver), destroy_player);
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

fn destroy_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q.iter() {
        commands.entity(player).despawn_recursive();
    }
}
