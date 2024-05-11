use bevy::prelude::*;

use crate::{
    assets::{GameSprites, ICON_INDEX_SCROLL_MARKER},
    enemy::{Enemy, EnemyBundle},
    inventory::{Scroll, ScrollUI},
    AppState,
};

const SCROLL_MARKER_TOP: f32 = -2.;
const SCROLL_MARKER_SPEED: f32 = 5.;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UseItem>()
            .insert_state(BattleState::default())
            .add_systems(OnEnter(AppState::Battling), setup_battle)
            .add_systems(OnExit(AppState::Battling), cleanup_battle)
            .add_systems(
                Update,
                (
                    player_turn_use_item,
                    update_scroll_marker_pos,
                    update_scroll_marker_ui_pos,
                    animate_scroll_marker,
                )
                    .chain()
                    .run_if(in_state(AppState::Battling)),
            );
    }
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum BattleState {
    #[default]
    Start,
    PlayerTurn,
    EnemyTurn,
    End,
}

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
    game_sprites: Res<GameSprites>,
    scroll_ui_q: Query<Entity, With<ScrollUI>>,
) {
    commands.spawn(EnemyBundle::default());

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
        .set_parent(scroll_ui_q.single());
}

fn player_turn_use_item(
    mut use_item_ew: EventWriter<UseItem>,
    key_codes: Res<ButtonInput<KeyCode>>,
    scroll: Res<Scroll>,
    scroll_marker_q: Query<&ScrollMarker>,
) {
    if key_codes.just_pressed(KeyCode::Space) {
        if let Ok(scroll_marker) = scroll_marker_q.get_single() {
            let Some(entity) = scroll.0.get(scroll_marker.0) else {
                return;
            };
            use_item_ew.send(UseItem(*entity));
        }
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
    scroll: Res<Scroll>,
) {
    let Ok(mut scroll_marker) = scroll_marker_q.get_single_mut() else {
        return;
    };
    for _ in use_item_er.read() {
        scroll_marker.0 = (scroll_marker.0 + 1) % scroll.0.len();
    }
}

fn update_scroll_marker_ui_pos(
    mut scroll_marker_q: Query<(&mut Style, &ScrollMarker), Changed<ScrollMarker>>,
) {
    for (mut style, scroll_marker) in scroll_marker_q.iter_mut() {
        let index = scroll_marker.0;
        style.left = Val::Px(index as f32 * 20.);
    }
}

fn cleanup_battle(mut commands: Commands, enemies_q: Query<Entity, With<Enemy>>) {
    for enemy_e in enemies_q.iter() {
        commands.get_entity(enemy_e).map(|e| e.despawn_recursive());
    }
}
