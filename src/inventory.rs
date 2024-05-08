use std::{collections::VecDeque, ops::Deref};

use bevy::{prelude::*, reflect::List, ui::RelativeCursorPosition, window::PrimaryWindow};
use bevy_trait_query::One;

use crate::{
    assets::GameSprites,
    items::{
        sword::{Sword, SwordType},
        Item, ItemPlugin,
    },
    AppState,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ItemPlugin)
            .add_systems(OnEnter(AppState::Game), (setup_inventory, open_gui).chain())
            .add_systems(
                Update,
                (start_dragging, stop_dragging, update_drag_container)
                    .chain()
                    .run_if(in_state(AppState::Game)),
            );
    }
}

const SCROLL_SIZE: usize = 12;

const ITEM_UI_SIZE: f32 = 16.;

#[derive(Component, Default)]
struct Scroll(VecDeque<Entity>);

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct ScrollUI;

#[derive(Component)]
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

fn setup_inventory(mut commands: Commands) {
    let mut scroll = Scroll::default();

    scroll.0.push_back(
        commands
            .spawn(Sword {
                r#type: SwordType::Wooden,
            })
            .id(),
    );
    scroll.0.push_back(
        commands
            .spawn(Sword {
                r#type: SwordType::Iron,
            })
            .id(),
    );
    scroll.0.push_back(
        commands
            .spawn(Sword {
                r#type: SwordType::Magic,
            })
            .id(),
    );
    commands.spawn(scroll);
}

fn open_gui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    scroll_q: Query<&Scroll>,
    items_q: Query<One<&dyn Item>>,
) {
    let scroll = scroll_q.single();

    let scroll_image = commands
        .spawn((
            ImageBundle {
                image: UiImage::new(game_sprites.inventory_scroll.clone()),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(4.),
                    width: Val::Px(245.),
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
            ScrollUI,
        ))
        .id();

    let background_image = commands
        .spawn(ImageBundle {
            image: UiImage::new(game_sprites.inventory_bg.clone()),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Px(260.),
                height: Val::Px(74.),
                padding: UiRect {
                    top: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .id();

    let root_node = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Px(4.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(scroll_image).with_children(|mut parent| {
        for &item_entity in scroll.0.iter() {
            let Ok(item) = items_q.get(item_entity) else {
                continue;
            };
            _spawn_ui_item(&mut parent, &game_sprites, item.deref(), item_entity);
        }
    });
    commands.entity(background_image).add_child(scroll_image);
    commands.entity(root_node).add_child(background_image);

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

fn start_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    drag_container_q: Query<Entity, With<DragContainer>>,
    draggables_q: Query<
        (Entity, &Parent, &RelativeCursorPosition),
        (With<Draggable>, Without<Dragging>),
    >,
) {
    for (entity, parent, relative_cursor_position) in draggables_q.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };
        if relative_cursor_position.mouse_over() && mouse.just_pressed(MouseButton::Left) {
            entity_commands.insert(Dragging {
                last_parent: parent.get(),
                last_index: 0,
            });
            entity_commands.set_parent(drag_container_q.single());
            return; // Can only drag one item at a time
        }
    }
}

fn stop_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    draggings_q: Query<(Entity, &Dragging)>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    for (entity, dragging) in draggings_q.iter() {
        let mut entity_commands = commands.entity(entity);

        entity_commands.set_parent(dragging.last_parent);
        entity_commands.remove::<Dragging>();
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

fn _spawn_ui_item(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    item: &dyn Item,
    item_entity: Entity,
) {
    parent.spawn((
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
        ItemUI(item_entity),
        Draggable,
        RelativeCursorPosition::default(),
    ));
}
