use bevy::prelude::*;

use crate::{
    assets::{GameSprites, ICON_INDEX_SCROLL_MARKER},
    common::Hp,
    enemy::{Enemy, ENEMY_DAMAGE},
    inventory::InventoryScrollUI,
    items::{
        abilities::{Ability, Damage, Heave, Jolly, SeaLegs, Squiffy},
        attributes::Attribute,
        Consumable,
    },
    log::LogMessageEvent,
    player::{Player, PlayerStats},
    AppState,
};

const SCROLL_MARKER_TOP: f32 = -2.;
const SCROLL_MARKER_SPEED: f32 = 5.;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleWins>()
            .insert_state(BattleState::PlayerTurn)
            .add_event::<UseItem>()
            .configure_sets(
                Update,
                (
                    PlayerTurnSet
                        .run_if(in_state(AppState::Battling))
                        .run_if(in_state(BattleState::PlayerTurn)),
                    EnemyTurnSet
                        .run_if(in_state(AppState::Battling))
                        .run_if(in_state(BattleState::EnemyTurn)),
                ),
            )
            .add_systems(OnEnter(AppState::Battling), setup_battle)
            .add_systems(OnExit(AppState::Battling), cleanup_battle)
            .add_systems(
                OnExit(BattleState::PlayerTurn),
                (on_player_turn_end,).run_if(in_state(AppState::Battling)),
            )
            .add_systems(
                OnExit(BattleState::EnemyTurn),
                (check_battle_end,).run_if(in_state(AppState::Battling)),
            )
            .add_systems(
                OnEnter(BattleState::EnemyTurn),
                enemy_turn.run_if(in_state(AppState::Battling)),
            )
            .add_systems(
                Update,
                (
                    player_turn_use_item,
                    handle_damage_use,
                    handle_jolly_use,
                    handle_squiffy_use,
                    handle_heave_use,
                    handle_sea_legs_use,
                    update_scroll_marker_pos,
                    update_scroll_marker_ui_pos,
                    animate_scroll_marker,
                    check_battle_end,
                )
                    .chain()
                    .in_set(PlayerTurnSet),
            );
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BattleState {
    PlayerTurn,
    EnemyTurn,
    BattleEnd,
}

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PlayerTurnSet;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EnemyTurnSet;

#[derive(Resource, Default)]
pub struct BattleWins(pub usize);

#[derive(Event)]
pub struct UseItem {
    pub item: Entity,
    pub consumed: bool,
}

#[derive(Component, Default)]
struct ScrollMarker(usize);

#[derive(Bundle, Default)]
struct ScrollMarkerBundle {
    scroll_marker: ScrollMarker,
    atlas_image_bundle: AtlasImageBundle,
}

fn setup_battle(
    mut commands: Commands,
    mut battle_state: ResMut<NextState<BattleState>>,
    game_sprites: Res<GameSprites>,
    scroll_ui_q: Query<&Children, With<InventoryScrollUI>>,
) {
    battle_state.set(BattleState::PlayerTurn);
    commands
        .spawn(ScrollMarkerBundle {
            atlas_image_bundle: AtlasImageBundle {
                image: UiImage::new(game_sprites.items_tile_sheet.clone()),
                texture_atlas: TextureAtlas {
                    layout: game_sprites.items_tile_layout.clone(),
                    index: ICON_INDEX_SCROLL_MARKER,
                },
                style: Style {
                    width: Val::Px(16.),
                    height: Val::Px(16.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(SCROLL_MARKER_TOP),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .set_parent(*scroll_ui_q.single().iter().next().unwrap());
}

fn player_turn_use_item(
    mut use_item_ew: EventWriter<UseItem>,
    mut battle_state: ResMut<NextState<BattleState>>,
    mut consumables_q: Query<&mut Consumable>,
    key_codes: Res<ButtonInput<KeyCode>>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
    scroll_marker_q: Query<&ScrollMarker>,
) {
    if key_codes.just_pressed(KeyCode::Space) {
        if let Ok(scroll_marker) = scroll_marker_q.get_single() {
            let Some(entity) = scroll_q.single().get(scroll_marker.0) else {
                println!("No items 0_0");
                battle_state.set(BattleState::EnemyTurn);
                return;
            };
            let mut consumed = false;
            if let Ok(mut consumable) = consumables_q.get_mut(*entity) {
                consumable.0 -= 1;
                consumable.0 = consumable.0.max(0);
                if consumable.0 == 0 {
                    consumed = true;
                }
            }
            use_item_ew.send(UseItem {
                item: *entity,
                consumed,
            });
            battle_state.set(BattleState::EnemyTurn);
        }
    }
}

fn on_player_turn_end(
    mut commands: Commands,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
    scroll_marker_q: Query<(Entity, &ScrollMarker)>,
    consumables_q: Query<&Consumable>,
) {
    let (scroll_m_e, scroll_marker) = scroll_marker_q.single();
    let children = scroll_q.single();
    let Some(used_item) = children.get(scroll_marker.0) else {
        return;
    };

    if let Ok(consumable) = consumables_q.get(*used_item) {
        if consumable.0 <= 0 {
            if children.len() <= 1 {
                commands.entity(scroll_m_e).remove_parent();
            } else {
                commands.entity(scroll_m_e).set_parent(
                    *children
                        .get((scroll_marker.0 + 1) % children.len())
                        .unwrap(),
                );
            }
            commands.entity(*used_item).despawn_recursive();
        }
    }
}

fn enemy_turn(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    mut battle_state: ResMut<NextState<BattleState>>,
    player_stats_q: Query<&PlayerStats>,
) {
    let player_stats = player_stats_q.single();
    let damage = (ENEMY_DAMAGE - player_stats.sea_legs).max(0);
    player_hp_q.single_mut().decrease(damage);
    log_message_ew.send(LogMessageEvent(format!(
        "Enemy dealt {} damage to Player!",
        damage
    )));
    battle_state.set(BattleState::PlayerTurn);
}

fn check_battle_end(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut battle_state: ResMut<NextState<BattleState>>,
    mut battle_wins: ResMut<BattleWins>,
    player_hp_q: Query<&Hp, With<Player>>,
    enemy_hp_q: Query<&Hp, With<Enemy>>,
) {
    println!("{}, {}", player_hp_q.single(), enemy_hp_q.single());
    if player_hp_q.single().is_dead() {
        next_app_state.set(AppState::GameOver);
        battle_state.set(BattleState::BattleEnd);
        println!("LOSE");
        return;
    }

    if enemy_hp_q.single().is_dead() {
        next_app_state.set(AppState::OrganizeInventory);
        battle_state.set(BattleState::BattleEnd);
        battle_wins.0 += 1;
        println!("WIN");
        return;
    }
}

fn animate_scroll_marker(
    mut scroll_marker_q: Query<&mut Style, With<ScrollMarker>>,
    time: Res<Time>,
) {
    for mut style in scroll_marker_q.iter_mut() {
        style.top =
            Val::Px(f32::sin(time.elapsed_seconds() * SCROLL_MARKER_SPEED) + SCROLL_MARKER_TOP);
    }
}

fn update_scroll_marker_pos(
    mut use_item_er: EventReader<UseItem>,
    mut scroll_marker_q: Query<&mut ScrollMarker>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok(mut scroll_marker) = scroll_marker_q.get_single_mut() else {
        return;
    };
    for UseItem { consumed, .. } in use_item_er.read() {
        if *consumed {
            continue;
        }
        scroll_marker.0 = (scroll_marker.0 + 1) % scroll_q.single().len();
    }
}

fn update_scroll_marker_ui_pos(
    mut commands: Commands,
    scroll_marker_q: Query<(Entity, &ScrollMarker), Changed<ScrollMarker>>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok((entity, scroll_marker)) = scroll_marker_q.get_single() else {
        return;
    };
    let index = scroll_marker.0;
    commands
        .entity(entity)
        .set_parent(*scroll_q.single().get(index).unwrap());
}

fn cleanup_battle(mut commands: Commands, scroll_marker_q: Query<Entity, With<ScrollMarker>>) {
    for entity in scroll_marker_q.iter() {
        commands.get_entity(entity).map(|e| e.despawn_recursive());
    }
}

fn handle_damage_use(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut enemy_hp_q: Query<&mut Hp, With<Enemy>>,
    damage_q: Query<&Damage>,
) {
    let Ok(mut enemy_hp) = enemy_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_er.read() {
        let Ok(damage) = damage_q.get(item_e.item) else {
            continue;
        };
        let amount = damage.amount();
        log_message_ew.send(LogMessageEvent(format!("Dealt {} damage!", amount)));
        enemy_hp.decrease(amount);
    }
}

fn handle_jolly_use(
    mut log_message_er: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    jolly_q: Query<&Jolly>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(jolly) = jolly_q.get(item_e.item) else {
            continue;
        };
        let amount = jolly.amount();
        log_message_er.send(LogMessageEvent(format!("Healed {} health!", amount)));
        player_hp.increase(amount);
    }
}

fn handle_squiffy_use(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    squiffy_q: Query<&Squiffy>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(squiffy) = squiffy_q.get(item_e.item) else {
            continue;
        };
        let amount = squiffy.amount();
        log_message_ew.send(LogMessageEvent(format!(
            "Self-inflicted {} health!",
            amount
        )));
        player_hp.decrease(amount);
    }
}

fn handle_heave_use(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut damage_q: Query<(&mut Damage, &dyn Attribute)>,
    heave_q: Query<&Heave>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let scroll_children = scroll_q.single();
    for item_e in use_item_er.read() {
        let Ok(heave) = heave_q.get(item_e.item) else {
            continue;
        };
        let i = scroll_children
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == item_e.item)
            .map(|(i, _)| i)
            .next();
        let Some(scroll_pos) = i else {
            continue;
        };
        let amount = heave.amount();
        let targets =
            heave
                .target
                .filter
                .get_targets(scroll_pos, item_e.item, scroll_children.iter());
        for &damage_e in targets.iter() {
            if let Ok((mut damage, attributes)) = damage_q.get_mut(damage_e) {
                if attributes
                    .iter()
                    .any(|a| a.name() == heave.target.attribute)
                {
                    damage.modifiers.entry(item_e.item).or_default().amount += amount;
                }
            }
        }
        log_message_ew.send(LogMessageEvent(format!("Heaved {}!", amount)));
    }
}

fn handle_sea_legs_use(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut player_stats_q: Query<&mut PlayerStats>,
    sea_legs_q: Query<&SeaLegs>,
) {
    for item_e in use_item_er.read() {
        let Ok(sea_legs) = sea_legs_q.get(item_e.item) else {
            continue;
        };
        let amount = sea_legs.amount();
        log_message_ew.send(LogMessageEvent(format!("Added {amount} Sea Legs!")));
        player_stats_q.single_mut().sea_legs += amount;
    }
}
