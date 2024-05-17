use bevy::prelude::*;

use crate::{
    assets::{GameFonts, GameSprites},
    battle::BattleState,
    common::Hp,
    ui::{BottomLeftUI, HealthBarUI, HealthBarUIText, FONT_COLOR, FONT_SIZE},
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
            PostUpdate,
            (
                update_player_hp_ui.run_if(any_with_component::<HealthBarUI>),
                update_player_stats_ui.run_if(any_with_component::<SeaLegsUI>),
            ),
        )
        .add_systems(OnEnter(AppState::Battling), reset_player_stats)
        .add_systems(OnExit(AppState::Battling), reset_player_stats)
        .add_systems(
            OnEnter(BattleState::PlayerTurn),
            update_player_stats.run_if(in_state(AppState::Battling)),
        );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerStats {
    pub sea_legs: i32,
}

#[derive(Component)]
struct SeaLegsUI;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    hp: Hp,
    player_stats: PlayerStats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            hp: Hp::new(STARTING_PLAYER_HP),
            player_stats: PlayerStats::default(),
        }
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn update_player_stats(mut player_stats_q: Query<&mut PlayerStats>) {
    let mut player_stats = player_stats_q.single_mut();
    player_stats.sea_legs = (player_stats.sea_legs - 1).max(0);
}

fn spawn_player_stats_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    player_stats_ui_q: Query<Entity, With<BottomLeftUI>>,
    player_stats_q: Query<&PlayerStats>,
    player_hp_q: Query<&Hp, With<Player>>,
) {
    let player_stats = player_stats_q.single();
    commands
        .entity(player_stats_ui_q.single())
        .with_children(|mut parent| {
            parent.spawn((
                Player,
                SeaLegsUI,
                TextBundle {
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "Sea Legs: ".to_string(),
                            style: TextStyle {
                                color: FONT_COLOR,
                                font_size: FONT_SIZE,
                                font: game_fonts.font.clone(),
                            },
                        },
                        TextSection {
                            value: format!("{}", player_stats.sea_legs),
                            style: TextStyle {
                                color: FONT_COLOR,
                                font_size: FONT_SIZE,
                                font: game_fonts.font.clone(),
                            },
                        },
                    ]),
                    ..default()
                },
            ));
            HealthBarUI::spawn(
                &mut parent,
                &game_sprites,
                &game_fonts,
                &player_hp_q.single(),
                Player,
            );
        });
}

fn reset_player_stats(mut player_stats_q: Query<&mut PlayerStats>) {
    *player_stats_q.single_mut() = PlayerStats::default();
}

fn update_player_hp_ui(
    mut health_bar_ui_q: Query<&mut TextureAtlas, (With<Player>, With<HealthBarUI>)>,
    mut hp_text_ui: Query<&mut Text, (With<Player>, With<HealthBarUIText>)>,
    player_hp_q: Query<&Hp, (With<Player>, Changed<Hp>)>,
) {
    if let Ok(hp) = player_hp_q.get_single() {
        health_bar_ui_q.single_mut().index = hp.health_bar_index();
        hp_text_ui.single_mut().sections.get_mut(0).unwrap().value = format!("{hp}");
    }
}

fn update_player_stats_ui(
    mut sea_legs_text_q: Query<&mut Text, (With<Player>, With<SeaLegsUI>)>,
    player_stats_q: Query<&PlayerStats, (With<Player>, Changed<PlayerStats>)>,
) {
    if let Ok(player_stats) = player_stats_q.get_single() {
        sea_legs_text_q
            .single_mut()
            .sections
            .get_mut(1)
            .unwrap()
            .value = format!("{}", player_stats.sea_legs);
    }
}

fn destroy_player(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q.iter() {
        commands.entity(player).despawn_recursive();
    }
}
