use bevy::prelude::*;

use crate::{
    assets::{GameFonts, GameSprites},
    battle::BattleWins,
    common::Hp,
    ui::{BottomRightUI, HealthBarUI, HealthBarUIText},
    AppState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleWins>()
            .add_systems(
                OnEnter(AppState::Battling),
                (spawn_enemy, spawn_enemy_stats_ui).chain(),
            )
            .add_systems(OnExit(AppState::Battling), (destroy_enemy,))
            .add_systems(
                Update,
                update_enemy_ui.run_if(any_with_component::<HealthBarUI>),
            );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub hp: Hp,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            hp: Hp::new(10),
        }
    }
}

impl EnemyBundle {
    fn from_battle_wins(battle_wins: &BattleWins) -> Self {
        let hp = 10 + battle_wins.0 * 5;

        Self {
            hp: Hp::new(hp as i32),
            ..default()
        }
    }
}

fn spawn_enemy(mut commands: Commands, battle_wins: Res<BattleWins>) {
    commands.spawn(EnemyBundle::from_battle_wins(&battle_wins));
}

fn spawn_enemy_stats_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    player_stats_ui_q: Query<Entity, With<BottomRightUI>>,
    player_hp_q: Query<&Hp, With<Enemy>>,
) {
    commands
        .entity(player_stats_ui_q.single())
        .with_children(|mut parent| {
            HealthBarUI::spawn(
                &mut parent,
                &game_sprites,
                &game_fonts,
                &player_hp_q.single(),
                Enemy,
            );
        });
}

fn update_enemy_ui(
    mut health_bar_ui: Query<&mut TextureAtlas, (With<Enemy>, With<HealthBarUI>)>,
    mut health_bar_ui_text: Query<&mut Text, (With<Enemy>, With<HealthBarUIText>)>,
    player_hp_q: Query<&Hp, (With<Enemy>, Changed<Hp>)>,
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

fn destroy_enemy(mut commands: Commands, enemies_q: Query<Entity, With<Enemy>>) {
    for entity in enemies_q.iter() {
        commands.get_entity(entity).map(|e| e.despawn_recursive());
    }
}
