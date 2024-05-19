use std::{borrow::BorrowMut, time::Duration};

use bevy::{animation::prelude, prelude::*};

use crate::{
    assets::{GameSprites, ICON_INDEX_SCROLL_MARKER},
    common::Hp,
    enemy::{Enemy, ENEMY_DAMAGE},
    inventory::InventoryScrollUI,
    items::{
        abilities::{Ability, Cursed, Damage, Hearties, Heave, Jolly, SeaLegs, Swashbuckle},
        attributes::{Attribute, Flintlock, Pellets},
        Consumable,
    },
    // log::LogMessageEvent,
    player::{Player, PlayerStats},
    AppState,
};

const SCROLL_MARKER_TOP: f32 = -2.;
const SCROLL_MARKER_SPEED: f32 = 5.;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleWins>()
            .add_event::<BattleEvent>()
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
            .add_systems(OnEnter(AppState::GameStart), reset_battle_wins)
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
                start_enemy_turn.run_if(in_state(AppState::Battling)),
            )
            .add_systems(Update, enemy_turn.in_set(EnemyTurnSet))
            .add_systems(
                Update,
                (
                    player_turn_use_item,
                    handle_damage_use,
                    handle_hearties_use,
                    handle_cursed_use,
                    handle_heave_use,
                    handle_sea_legs_use,
                    handle_swashbuckle_use,
                    handle_jolly_use,
                    handle_pellets_use,
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

#[derive(Event, Clone, Copy)]
pub enum BattleEvent {
    PlayerHurt(i32),
    PlayerHeal(i32),
    EnemyHurt(i32),
    EnemyAttack,
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

#[derive(Component)]
struct EnemyTurnTimer(Timer);

#[derive(Component, Default)]
struct ScrollMarker(usize);

#[derive(Bundle, Default)]
struct ScrollMarkerBundle {
    scroll_marker: ScrollMarker,
    atlas_image_bundle: AtlasImageBundle,
}

fn reset_battle_wins(mut commands: Commands) {
    commands.insert_resource(BattleWins::default());
}

fn setup_battle(
    mut commands: Commands,
    mut battle_state: ResMut<NextState<BattleState>>,
    game_sprites: Res<GameSprites>,
    scroll_ui_q: Query<&Children, With<InventoryScrollUI>>,
) {
    battle_state.set(BattleState::PlayerTurn);
    let scroll_marker_ui = commands
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
        .id();
    if let Ok(children) = scroll_ui_q.get_single() {
        if let Some(item) = children.iter().next() {
            commands.entity(scroll_marker_ui).set_parent(*item);
        }
    }
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
        let Ok(children) = scroll_q.get_single() else {
            battle_state.set(BattleState::EnemyTurn);
            return;
        };
        if let Ok(scroll_marker) = scroll_marker_q.get_single() {
            let Some(entity) = children.get(scroll_marker.0) else {
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
    let Ok(children) = scroll_q.get_single() else {
        return;
    };
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

fn start_enemy_turn(mut commands: Commands) {
    commands.spawn(EnemyTurnTimer(Timer::new(
        Duration::from_secs_f32(0.5),
        TimerMode::Once,
    )));
}

fn enemy_turn(
    mut commands: Commands,
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut battle_event_ew: EventWriter<BattleEvent>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    mut battle_state: ResMut<NextState<BattleState>>,
    mut turn_timer_q: Query<(Entity, &mut EnemyTurnTimer)>,
    player_stats_q: Query<&PlayerStats>,
    time: Res<Time>,
) {
    let (entity, mut turn_timer) = turn_timer_q.single_mut();
    turn_timer.0.tick(time.delta());
    if turn_timer.0.just_finished() {
        let player_stats = player_stats_q.single();
        let damage = (ENEMY_DAMAGE - player_stats.sea_legs).max(0);
        player_hp_q.single_mut().decrease(damage);
        battle_event_ew.send(BattleEvent::EnemyAttack);
        battle_event_ew.send(BattleEvent::PlayerHurt(damage));
        // log_message_ew.send(LogMessageEvent(format!(
        //     "Enemy dealt {} damage to Player!",
        //     damage
        // )));
        battle_state.set(BattleState::PlayerTurn);
        commands.entity(entity).despawn_recursive();
    }
}

fn check_battle_end(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut battle_state: ResMut<NextState<BattleState>>,
    mut battle_wins: ResMut<BattleWins>,
    player_hp_q: Query<&Hp, With<Player>>,
    enemy_hp_q: Query<&Hp, With<Enemy>>,
) {
    if player_hp_q.single().is_dead() {
        next_app_state.set(AppState::GameOver);
        battle_state.set(BattleState::BattleEnd);
        return;
    }

    if enemy_hp_q.single().is_dead() {
        next_app_state.set(AppState::OrganizeInventory);
        battle_state.set(BattleState::BattleEnd);
        battle_wins.0 += 1;
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
    let Ok(children) = scroll_q.get_single() else {
        return;
    };
    if children.len() > 0 {
        commands
            .entity(entity)
            .set_parent(*children.get(index).unwrap());
    }
}

fn cleanup_battle(mut commands: Commands, scroll_marker_q: Query<Entity, With<ScrollMarker>>) {
    for entity in scroll_marker_q.iter() {
        commands.get_entity(entity).map(|e| e.despawn_recursive());
    }
}

fn handle_damage_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut battle_event_ew: EventWriter<BattleEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut enemy_hp_q: Query<&mut Hp, With<Enemy>>,
    mut damage_q: Query<(&Damage, Option<&mut Flintlock>)>,
) {
    let Ok(mut enemy_hp) = enemy_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_er.read() {
        let Ok((damage, flintlock)) = damage_q.get_mut(item_e.item) else {
            continue;
        };
        if let Some(mut flintlock) = flintlock {
            if !flintlock.fire() {
                continue;
            }
        }
        let amount = damage.amount();
        battle_event_ew.send(BattleEvent::EnemyHurt(amount));
        // log_message_ew.send(LogMessageEvent(format!("Dealt {} damage!", amount)));
        enemy_hp.decrease(amount);
    }
}

fn handle_hearties_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut battle_event_ew: EventWriter<BattleEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    hearties_q: Query<&Hearties>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(hearties) = hearties_q.get(item_e.item) else {
            continue;
        };
        let amount = hearties.amount();
        battle_event_ew.send(BattleEvent::PlayerHeal(amount));
        // log_message_ew.send(LogMessageEvent(format!("Healed {} health!", amount)));
        player_hp.increase(amount);
    }
}

fn handle_cursed_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut battle_event_ew: EventWriter<BattleEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    cursed_q: Query<&Cursed>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(cursed) = cursed_q.get(item_e.item) else {
            continue;
        };
        let amount = cursed.amount();
        battle_event_ew.send(BattleEvent::PlayerHurt(amount));
        // log_message_ew.send(LogMessageEvent(format!(
        //     "Self-inflicted {} health!",
        //     amount
        // )));
        player_hp.decrease(amount);
    }
}

fn handle_heave_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut damage_q: Query<(&mut Damage, &dyn Attribute)>,
    heave_q: Query<&Heave>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok(scroll_children) = scroll_q.get_single() else {
        return;
    };
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
                    .any(|a| a.name().contains(&heave.target.attribute))
                {
                    damage.modifier.amount += amount;
                }
            }
        }
        // log_message_ew.send(LogMessageEvent(format!("Heaved {}!", amount)));
    }
}

fn handle_sea_legs_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut player_stats_q: Query<&mut PlayerStats>,
    sea_legs_q: Query<&SeaLegs>,
) {
    for item_e in use_item_er.read() {
        let Ok(sea_legs) = sea_legs_q.get(item_e.item) else {
            continue;
        };
        let amount = sea_legs.amount();
        // log_message_ew.send(LogMessageEvent(format!("Added {amount} Sea Legs!")));
        player_stats_q.single_mut().sea_legs += amount;
    }
}

fn handle_swashbuckle_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut sea_legs_q: Query<(&mut SeaLegs, &dyn Attribute)>,
    swashbuckle_q: Query<&Swashbuckle>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok(scroll_children) = scroll_q.get_single() else {
        return;
    };
    for item_e in use_item_er.read() {
        let Ok(swashbuckle) = swashbuckle_q.get(item_e.item) else {
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
        let amount = swashbuckle.amount();
        let targets =
            swashbuckle
                .target
                .filter
                .get_targets(scroll_pos, item_e.item, scroll_children.iter());
        for &sea_legs_e in targets.iter() {
            if let Ok((mut sea_legs, attributes)) = sea_legs_q.get_mut(sea_legs_e) {
                if attributes
                    .iter()
                    .any(|a| a.name().contains(&swashbuckle.target.attribute))
                {
                    sea_legs.modifier.amount += amount;
                }
            }
        }
        // log_message_ew.send(LogMessageEvent(format!("Heaved {}!", amount)));
    }
}

fn handle_jolly_use(
    // mut log_message_ew: EventWriter<LogMessageEvent>,
    mut use_item_er: EventReader<UseItem>,
    mut hearties_q: Query<(&mut Hearties, &dyn Attribute)>,
    jolly_q: Query<&Jolly>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok(scroll_children) = scroll_q.get_single() else {
        return;
    };
    for item_e in use_item_er.read() {
        let Ok(jolly) = jolly_q.get(item_e.item) else {
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
        let amount = jolly.amount();
        let targets =
            jolly
                .target
                .filter
                .get_targets(scroll_pos, item_e.item, scroll_children.iter());
        for &hearties_e in targets.iter() {
            if let Ok((mut hearties, attributes)) = hearties_q.get_mut(hearties_e) {
                if attributes
                    .iter()
                    .any(|a| a.name().contains(&jolly.target.attribute))
                {
                    hearties.modifier.amount += amount;
                }
            }
        }
        // log_message_ew.send(LogMessageEvent(format!("Heaved {}!", amount)));
    }
}

fn handle_pellets_use(
    mut use_item_er: EventReader<UseItem>,
    mut flintlock_q: Query<&mut Flintlock>,
    pellets_q: Query<&Pellets>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let Ok(scroll_children) = scroll_q.get_single() else {
        return;
    };
    for item_e in use_item_er.read() {
        let Ok(pellets) = pellets_q.get(item_e.item) else {
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
        let targets =
            pellets
                .target
                .filter
                .get_targets(scroll_pos, item_e.item, scroll_children.iter());
        for &flintlock_e in targets.iter() {
            if let Ok(mut flintlock) = flintlock_q.get_mut(flintlock_e) {
                if flintlock.can_load(&pellets.name().to_string()) {
                    flintlock.load(pellets.load_amount);
                }
            }
        }
    }
}
