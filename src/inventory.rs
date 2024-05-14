use bevy::{prelude::*, ui::RelativeCursorPosition, window::PrimaryWindow};

use crate::{
    assets::GameSprites,
    items::{
        sword::{Sword, SwordType},
        Item,
    },
    tooltip::Tooltipable,
    ui::RootUINode,
    AppState,
};

pub const INVENTORY_SCROLL_SIZE: usize = 12;
pub const LOOT_SCROLL_SIZE: usize = 5;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameStart), (open_gui).chain())
            .add_systems(
                OnEnter(AppState::OrganizeInventory),
                (spawn_loot_scroll_ui, spawn_loot).chain(),
            )
            .add_systems(OnExit(AppState::OrganizeInventory), destroy_loot_scroll_ui)
            .add_systems(
                Update,
                (start_dragging, stop_dragging, update_drag_container)
                    .chain()
                    .run_if(in_state(AppState::OrganizeInventory)),
            );
    }
}

const INVENTORY_SCROLL_UI_WIDTH: f32 = 245.;
const LOOT_SCROLL_UI_WIDTH: f32 = 105.;
const ITEM_UI_SIZE: f32 = 16.;

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct ScrollUI {
    size: usize,
}

#[derive(Component)]
pub struct InventoryScrollUI;

#[derive(Component)]
pub struct LootScrollUI;

#[derive(Component, Clone, Copy)]
struct ItemUI(Entity);

#[derive(Component)]
struct DragContainer;

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct Dragging {
    last_parent: Entity,
    last_index: usize,
}

fn spawn_loot(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    loot_scroll_q: Query<Entity, With<LootScrollUI>>,
) {
    commands
        .entity(loot_scroll_q.single())
        .with_children(|parent| {
            _spawn_ui_item(
                parent,
                &game_sprites,
                &Sword {
                    r#type: SwordType::Blessed,
                },
            );
        });
}

fn open_gui(
    mut commands: Commands,
    root_ui_node_q: Query<Entity, With<RootUINode>>,
    game_sprites: Res<GameSprites>,
) {
    let scroll_image = commands
        .spawn((
            InventoryScrollUI,
            ScrollUI {
                size: INVENTORY_SCROLL_SIZE,
            },
            ImageBundle {
                image: UiImage::new(game_sprites.inventory_scroll.clone()),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(4.),
                    width: Val::Px(INVENTORY_SCROLL_UI_WIDTH),
                    height: Val::Px(25.),
                    padding: UiRect {
                        left: Val::Px(2.),
                        right: Val::Px(3.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            RelativeCursorPosition::default(),
        ))
        .id();

    let background_image = commands
        .spawn((
            InventoryUI,
            ImageBundle {
                image: UiImage::new(game_sprites.inventory_bg.clone()),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    width: Val::Px(260.),
                    height: Val::Px(74.),
                    padding: UiRect {
                        top: Val::Px(8.),
                        bottom: Val::Px(8.),
                        ..default()
                    },
                    row_gap: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands.entity(background_image).add_child(scroll_image);
    commands
        .entity(root_ui_node_q.single())
        .add_child(background_image);

    commands.spawn((
        DragContainer,
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_loot_scroll_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    inventory_ui_q: Query<Entity, With<InventoryUI>>,
) {
    let loot_scroll_ui = commands
        .spawn((
            LootScrollUI,
            ScrollUI {
                size: LOOT_SCROLL_SIZE,
            },
            ImageBundle {
                image: UiImage::new(game_sprites.loot_scroll.clone()),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(4.),
                    width: Val::Px(LOOT_SCROLL_UI_WIDTH),
                    height: Val::Px(25.),
                    padding: UiRect {
                        left: Val::Px(2.),
                        right: Val::Px(3.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            RelativeCursorPosition::default(),
        ))
        .id();

    commands
        .entity(inventory_ui_q.single())
        .add_child(loot_scroll_ui);
}

fn destroy_loot_scroll_ui(
    mut commands: Commands,
    loot_scroll_ui_q: Query<Entity, With<LootScrollUI>>,
) {
    for loot_scroll_e in loot_scroll_ui_q.iter() {
        commands.entity(loot_scroll_e).despawn_recursive();
    }
}

fn start_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    scroll_ui_children_q: Query<&Children, With<ScrollUI>>,
    drag_container_q: Query<Entity, With<DragContainer>>,
    mut draggables_q: Query<
        (
            Entity,
            &Parent,
            &RelativeCursorPosition,
            Option<&mut Tooltipable>,
        ),
        (With<Draggable>, Without<Dragging>),
    >,
) {
    for (entity, parent, relative_cursor_position, tooltipable) in draggables_q.iter_mut() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };
        if relative_cursor_position.mouse_over() && mouse.just_pressed(MouseButton::Left) {
            let mut index = 0;
            if let Ok(children) = scroll_ui_children_q.get(parent.get()) {
                let i = children
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| c == entity)
                    .map(|(i, _)| i)
                    .next();
                if let Some(i) = i {
                    index = i % children.len();
                }
            }
            if let Some(mut t) = tooltipable {
                *t = Tooltipable::Disabled;
            }
            entity_commands.insert(Dragging {
                last_parent: parent.get(),
                last_index: index,
            });
            entity_commands.set_parent(drag_container_q.single());
            return; // Can only drag one item at a time
        }
    }
}

fn stop_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut draggings_q: Query<(Entity, &Dragging, Option<&mut Tooltipable>)>,
    scroll_ui_q: Query<(
        Entity,
        &RelativeCursorPosition,
        &ScrollUI,
        Option<&Children>,
    )>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    for (drag_entity, dragging, tooltipable) in draggings_q.iter_mut() {
        let mut index = dragging.last_index;
        let mut parent = dragging.last_parent;
        if let Some((parent_e, relative_cursor_position, scroll_ui, children)) = scroll_ui_q
            .iter()
            .filter(|(_, rcp, _, _)| rcp.mouse_over())
            .next()
        {
            if let Some(norm) = relative_cursor_position.normalized {
                if children.map_or(0, |c| c.len()) < scroll_ui.size {
                    index = (norm.x * scroll_ui.size as f32) as usize;
                    index = usize::min(index, children.map_or(0, |c| c.len()));
                    parent = parent_e;
                }
            }
        }
        let mut parent_commands = commands.entity(parent);
        if let Some(mut t) = tooltipable {
            *t = Tooltipable::Enabled;
        }
        parent_commands.insert_children(index, &[drag_entity]);
        commands.entity(drag_entity).remove::<Dragging>();
    }
}

fn update_drag_container(
    mut drag_container_style_q: Query<&mut Style, With<DragContainer>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(position) = windows_q.single().cursor_position() else {
        return;
    };

    let mut style = drag_container_style_q.single_mut();
    style.left = Val::Px(position.x - ITEM_UI_SIZE * 0.5);
    style.top = Val::Px(position.y - ITEM_UI_SIZE * 0.5);
}

fn _spawn_ui_item(parent: &mut ChildBuilder, game_sprites: &GameSprites, item: &dyn Item) {
    let mut entity_commands = parent.spawn((
        AtlasImageBundle {
            image: UiImage::new(game_sprites.items_tile_sheet.clone()),
            texture_atlas: TextureAtlas {
                layout: game_sprites.items_tile_layout.clone(),
                index: item.icon_id(),
            },
            style: Style {
                width: Val::Px(16.),
                height: Val::Px(16.),
                ..default()
            },
            ..default()
        },
        Draggable,
        RelativeCursorPosition::default(),
        Tooltipable::default(),
    ));

    item.add_bundle(&mut entity_commands);
}
