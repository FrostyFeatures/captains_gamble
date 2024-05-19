use bevy::{prelude::*, ui::RelativeCursorPosition, window::PrimaryWindow};

use crate::{
    assets::{GameFonts, GameSprites},
    items::{Item, ItemType},
    tooltip::Tooltipable,
    ui::{BottomCenterUI, BottomRightUI, TopInventoryUI, FONT_COLOR},
    AppState,
};

pub const INVENTORY_SCROLL_SIZE: usize = 12;
pub const LOOT_SCROLL_SIZE: usize = 5;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::InitGame), spawn_inventory_scroll)
            .add_systems(OnExit(AppState::GameOver), cleanup_inventory_scroll)
            .add_systems(
                OnEnter(AppState::OrganizeInventory),
                (spawn_loot_scroll_ui, spawn_loot, spawn_start_battle_button).chain(),
            )
            .add_systems(
                OnExit(AppState::OrganizeInventory),
                (destroy_loot_scroll_ui, destroy_buttons),
            )
            .add_systems(
                Update,
                (start_dragging, stop_dragging, update_drag_container)
                    .chain()
                    .run_if(in_state(AppState::OrganizeInventory)),
            )
            .add_systems(
                Update,
                button_system.run_if(any_with_component::<StartBattleButton>),
            );
    }
}

const INVENTORY_SCROLL_UI_WIDTH: f32 = 245.;
const LOOT_SCROLL_UI_WIDTH: f32 = 105.;
const ITEM_UI_SIZE: f32 = 16.;

#[derive(Component)]
struct ScrollUI {
    size: usize,
}

#[derive(Component)]
pub struct InventoryScrollUI;

#[derive(Component)]
pub struct LootScrollUI;

#[derive(Component)]
struct StartBattleButton;

#[derive(Component, Clone, Copy)]
struct ItemUI(Entity);

#[derive(Component)]
struct DragContainer;

#[derive(Component)]
pub struct Draggable;

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
            ItemType::WoodenSword.spawn(parent, &game_sprites);
            ItemType::Grog.spawn(parent, &game_sprites);
            ItemType::Orange.spawn(parent, &game_sprites);
            ItemType::Spyglass.spawn(parent, &game_sprites);
            ItemType::BlessedCutlass.spawn(parent, &game_sprites);
        });
}

fn spawn_inventory_scroll(
    mut commands: Commands,
    top_inventory_ui_q: Query<Entity, With<TopInventoryUI>>,
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
                    min_width: Val::Px(INVENTORY_SCROLL_UI_WIDTH),
                    max_width: Val::Px(INVENTORY_SCROLL_UI_WIDTH),
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
        .entity(top_inventory_ui_q.single())
        .add_child(scroll_image);

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

fn cleanup_inventory_scroll(mut commands: Commands, scroll_ui_q: Query<&Children, With<ScrollUI>>) {
    let Ok(children) = scroll_ui_q.get_single() else {
        return;
    };

    for child in children.iter() {
        commands.entity(*child).despawn_recursive();
    }
}

fn spawn_loot_scroll_ui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    bottom_inventory_ui_q: Query<Entity, With<BottomCenterUI>>,
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
                    max_width: Val::Px(LOOT_SCROLL_UI_WIDTH),
                    min_width: Val::Px(LOOT_SCROLL_UI_WIDTH),
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
        .entity(bottom_inventory_ui_q.single())
        .add_child(loot_scroll_ui);
}

fn spawn_start_battle_button(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    bottom_right_ui_q: Query<Entity, With<BottomRightUI>>,
) {
    let start_battle_button = commands
        .spawn((
            StartBattleButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(65.),
                    height: Val::Px(16.),
                    padding: UiRect {
                        left: Val::Px(4.),
                        top: Val::Px(6.),
                        ..default()
                    },
                    ..default()
                },
                image: game_sprites.start_battle_button.clone().into(),
                ..default()
            },
        ))
        .id();

    let button_text = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Start Battle",
                TextStyle {
                    color: FONT_COLOR,
                    font_size: 7.,
                    font: game_fonts.font.clone(),
                },
            ),
            ..default()
        })
        .id();

    commands.entity(start_battle_button).add_child(button_text);
    commands
        .entity(bottom_right_ui_q.single())
        .add_child(start_battle_button);
}

fn destroy_loot_scroll_ui(
    mut commands: Commands,
    loot_scroll_ui_q: Query<Entity, With<LootScrollUI>>,
) {
    for loot_scroll_e in loot_scroll_ui_q.iter() {
        commands.entity(loot_scroll_e).despawn_recursive();
    }
}

fn destroy_buttons(mut commands: Commands, buttons_q: Query<Entity, With<StartBattleButton>>) {
    for button in buttons_q.iter() {
        commands.entity(button).despawn_recursive();
    }
}

fn button_system(
    mut interaction_q: Query<(&Interaction, &mut UiImage), With<StartBattleButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    game_sprites: Res<GameSprites>,
) {
    let (interaction, mut image) = interaction_q.single_mut();
    match *interaction {
        Interaction::Pressed => app_state.set(AppState::Battling),
        Interaction::Hovered => image.texture = game_sprites.start_battle_button_hover.clone(),
        Interaction::None => image.texture = game_sprites.start_battle_button.clone(),
    };
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
