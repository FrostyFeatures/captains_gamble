
use std::{collections::VecDeque, ops::Deref};

use bevy::{prelude::*, ui::RelativeCursorPosition, window::PrimaryWindow};
use bevy_trait_query::One;

use crate::{
    assets::GameSprites,
    items::{sword::{Sword, SwordType}, Item, ItemPlugin}
    ,
    AppState,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ItemPlugin)
            .add_systems(OnEnter(AppState::Game), (
                setup_inventory,
                open_gui
            ).chain())
            .add_systems(Update, (
                start_dragging,
                stop_dragging,
                update_dragging,
            ).chain().run_if(in_state(AppState::Game)));
    }
}


const SCROLL_SIZE: usize = 12;

#[derive(Component, Default)]
struct Scroll(VecDeque<Entity>);

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct ScrollUI;

#[derive(Component)]
struct ItemUI(Entity);

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct Dragging;


fn setup_inventory(
    mut commands: Commands,
) {
    let scroll_entity = commands.spawn(Scroll::default()).id();

    commands.entity(scroll_entity).with_children(|parent| {
        parent.spawn(Sword {
            r#type: SwordType::Wooden,
        });
        parent.spawn(Sword {
            r#type: SwordType::Iron,
        });
        parent.spawn(Sword {
            r#type: SwordType::Magic,
        });
    });
}

fn open_gui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    scroll_q: Query<&Children, With<Scroll>>,
    items_q: Query<One<&dyn Item>>,
) {
    let scroll_children = scroll_q.single();

    let scroll_image = commands.spawn((
        ImageBundle {
            image: UiImage::new(game_sprites.inventory_scroll.clone()),
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                width: Val::Px(245.),
                height: Val::Px(25.),
                padding: UiRect {
                    left: Val::Px(2.),
                    right: Val::Px(3.),
                    ..default()
                },
                row_gap: Val::Px(4.),
                ..default()
            },
            ..default()
        },
        ScrollUI,
    )).id();

    let background_image = commands.spawn(ImageBundle {
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
    }).id();

    let root_node = commands.spawn(NodeBundle {
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
    }).id();


    commands.entity(scroll_image).with_children(|mut parent| {
        for &item_entity in scroll_children.iter() {
            let Ok(item) = items_q.get(item_entity) else {
                continue;
            };
            _spawn_ui_item(&mut parent, &game_sprites, item.deref(), item_entity);
        }
    });
    commands.entity(background_image).add_child(scroll_image);
    commands.entity(root_node).add_child(background_image);
}

fn start_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    draggables_q: Query<(Entity, &RelativeCursorPosition), (With<Draggable>, Without<Dragging>)>,
) {
    for (entity,relative_cursor_position) in draggables_q.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };
        if relative_cursor_position.mouse_over() && mouse.just_pressed(MouseButton::Left) {
            entity_commands.insert(Dragging);
            entity_commands.remove_parent();
            return; // Can only drag one item at a time
        }
    }
}

fn stop_dragging(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    draggings_q: Query<Entity, With<Dragging>>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    for entity in draggings_q.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };

        entity_commands.remove::<Dragging>();
    }
}

fn update_dragging(
    mut draggings_q: Query<&mut Style, With<Dragging>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(position) = windows_q.single().cursor_position() else {
        return;
    };

    for mut style in draggings_q.iter_mut() {
        style.left = Val::Px(position.x - 8.);
        style.top = Val::Px(position.y - 8.);
    }
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

