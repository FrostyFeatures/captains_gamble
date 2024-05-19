use bevy::prelude::*;

use crate::{
    assets::{GameFonts, GameSprites},
    common::Hp,
    items::abilities::{Ability, Damage},
    ui::{BottomRightUI, HealthBarUI, HealthBarUIText, FONT_COLOR, FONT_SIZE},
    AppState, BattleWins,
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
                PostUpdate,
                (
                    update_enemy_hp_ui.run_if(any_with_component::<HealthBarUI>),
                    update_enemy_damage_ui.run_if(any_with_component::<EnemyDamageUI>),
                ),
            );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Enemy;

#[derive(Component, Default, Clone, Copy)]
pub struct EnemyDamageUI;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub hp: Hp,
    pub damage: Damage,
}

impl EnemyBundle {
    fn from_battle_wins(battle_wins: &BattleWins) -> Self {
        let hp = Hp::new(6 + 3 * (battle_wins.0 as f32).powf(1.1) as i32);
        let damage = Damage::new((3 as f32 + battle_wins.0 as f32 * 0.1) as i32);

        Self {
            enemy: Enemy,
            hp,
            damage,
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
    enemy_stats_ui_q: Query<Entity, With<BottomRightUI>>,
    enemy_stats_q: Query<(&Hp, &Damage), With<Enemy>>,
) {
    let (hp, damage) = enemy_stats_q.single();
    commands
        .entity(enemy_stats_ui_q.single())
        .with_children(|mut parent| {
            parent.spawn((
                Enemy,
                EnemyDamageUI,
                TextBundle {
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "Damage: ".to_string(),
                            style: TextStyle {
                                color: FONT_COLOR,
                                font_size: FONT_SIZE,
                                font: game_fonts.font.clone(),
                            },
                        },
                        TextSection {
                            value: format!("{}", damage.amount()),
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
            HealthBarUI::spawn(&mut parent, &game_sprites, &game_fonts, &hp, Enemy);
        });
}

fn update_enemy_hp_ui(
    mut health_bar_ui: Query<&mut TextureAtlas, (With<Enemy>, With<HealthBarUI>)>,
    mut health_bar_ui_text: Query<&mut Text, (With<Enemy>, With<HealthBarUIText>)>,
    enemy_hp_q: Query<&Hp, With<Enemy>>,
) {
    if let Ok(hp) = enemy_hp_q.get_single() {
        health_bar_ui.single_mut().index = hp.health_bar_index();
        health_bar_ui_text
            .single_mut()
            .sections
            .get_mut(0)
            .unwrap()
            .value = format!("{hp}");
    }
}

fn update_enemy_damage_ui(
    mut enemy_damage_ui_text: Query<&mut Text, (With<Enemy>, With<EnemyDamageUI>)>,
    enemy_damage_q: Query<&Damage, With<Enemy>>,
) {
    if let Ok(damage) = enemy_damage_q.get_single() {
        enemy_damage_ui_text
            .single_mut()
            .sections
            .get_mut(1)
            .unwrap()
            .value = format!("{}", damage.amount());
    }
}

fn destroy_enemy(mut commands: Commands, enemies_q: Query<Entity, With<Enemy>>) {
    for entity in enemies_q.iter() {
        commands.get_entity(entity).map(|e| e.despawn_recursive());
    }
}
