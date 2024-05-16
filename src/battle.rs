use bevy::{prelude::*, render::batching::batch_and_prepare_render_phase};

use crate::{
    assets::{GameSprites, ICON_INDEX_SCROLL_MARKER},
    common::Hp,
    enemy::{Enemy, ENEMY_DAMAGE},
    inventory::InventoryScrollUI,
    log::LogMessageEvent,
    player::Player,
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
                Update,
                (
                    (
                        update_scroll_marker_pos,
                        update_scroll_marker_ui_pos,
                        animate_scroll_marker,
                        check_battle_end,
                    )
                        .chain()
                        .run_if(in_state(AppState::Battling)),
                    player_turn_use_item.in_set(PlayerTurnSet),
                ),
            )
            .add_systems(OnEnter(BattleState::EnemyTurn), enemy_turn);
    }
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BattleState {
    PlayerTurn,
    EnemyTurn,
}

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PlayerTurnSet;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EnemyTurnSet;

#[derive(Resource, Default)]
pub struct BattleWins(pub usize);

#[derive(Event)]
pub struct UseItem(pub Entity);

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
    key_codes: Res<ButtonInput<KeyCode>>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
    scroll_marker_q: Query<&ScrollMarker>,
) {
    if key_codes.just_pressed(KeyCode::Space) {
        if let Ok(scroll_marker) = scroll_marker_q.get_single() {
            let Some(entity) = scroll_q.single().get(scroll_marker.0) else {
                return;
            };
            use_item_ew.send(UseItem(*entity));
            battle_state.set(BattleState::EnemyTurn);
        }
    }
}

fn enemy_turn(
    mut log_message_ew: EventWriter<LogMessageEvent>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    mut battle_state: ResMut<NextState<BattleState>>,
) {
    player_hp_q.single_mut().decrease(ENEMY_DAMAGE);
    log_message_ew.send(LogMessageEvent(format!(
        "Enemy dealt {} damage to Player!",
        ENEMY_DAMAGE
    )));
    battle_state.set(BattleState::PlayerTurn);
}

fn check_battle_end(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut battle_wins: ResMut<BattleWins>,
    player_hp_q: Query<&Hp, With<Player>>,
    enemy_hp_q: Query<&Hp, With<Enemy>>,
) {
    if player_hp_q.single().is_dead() {
        next_app_state.set(AppState::GameOver);
    }

    if enemy_hp_q.single().is_dead() {
        next_app_state.set(AppState::OrganizeInventory);
        battle_wins.0 += 1;
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
    for _ in use_item_er.read() {
        scroll_marker.0 = (scroll_marker.0 + 1) % scroll_q.single().len();
    }
}

fn update_scroll_marker_ui_pos(
    mut commands: Commands,
    scroll_marker_q: Query<(Entity, &ScrollMarker), Changed<ScrollMarker>>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    for (entity, scroll_marker) in scroll_marker_q.iter() {
        let index = scroll_marker.0;
        commands
            .entity(entity)
            .set_parent(*scroll_q.single().get(index).unwrap());
    }
}

fn cleanup_battle(mut commands: Commands, scroll_marker_q: Query<Entity, With<ScrollMarker>>) {
    for entity in scroll_marker_q.iter() {
        commands.get_entity(entity).map(|e| e.despawn_recursive());
    }
}
