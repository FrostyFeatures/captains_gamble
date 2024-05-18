use bevy::prelude::*;

use crate::{
    assets::{GameFonts, GameSprites},
    common::Hp,
    AppState,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitGame), setup_root_node)
            .add_systems(OnEnter(AppState::GameStart), spawn_start_game_button)
            .add_systems(OnExit(AppState::GameStart), destroy_start_button)
            .add_systems(
                Update,
                start_button_system.run_if(any_with_component::<StartGameButton>),
            )
            .add_systems(OnEnter(AppState::GameOver), spawn_restart_game_button)
            .add_systems(OnExit(AppState::GameOver), destroy_restart_button)
            .add_systems(
                Update,
                restart_button_system.run_if(any_with_component::<RestartGameButton>),
            );
    }
}

pub const FONT_SIZE: f32 = 6.;
pub const FONT_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct RootUINode;

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct TopInventoryUI;

#[derive(Component)]
pub struct BottomInventoryUI;

#[derive(Component)]
pub struct BottomLeftUI;

#[derive(Component)]
pub struct BottomCenterUI;

#[derive(Component)]
pub struct BottomRightUI;

#[derive(Component)]
pub struct HealthBarUIText;

#[derive(Component)]
pub struct HealthBarUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct RestartGameButton;

impl HealthBarUI {
    pub fn spawn(
        parent: &mut ChildBuilder,
        game_sprites: &GameSprites,
        game_fonts: &GameFonts,
        hp: &Hp,
        tag: impl Component + Copy,
    ) {
        parent.spawn((
            tag,
            HealthBarUI,
            AtlasImageBundle {
                image: UiImage::new(game_sprites.health_bar_sheet.clone()),
                texture_atlas: TextureAtlas {
                    layout: game_sprites.health_bar_layout.clone(),
                    index: hp.health_bar_index(),
                },
                ..default()
            },
        ));

        parent
            .spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    width: Val::Px(67.),
                    height: Val::Px(9.),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    tag,
                    HealthBarUIText,
                    TextBundle {
                        text: Text::from_section(
                            format!("{hp}"),
                            TextStyle {
                                color: FONT_COLOR,
                                font_size: FONT_SIZE,
                                font: game_fonts.font.clone(),
                            },
                        ),
                        style: Style { ..default() },
                        ..default()
                    },
                ));
            });
    }
}

fn setup_root_node(mut commands: Commands, game_sprites: Res<GameSprites>) {
    let root_node = commands
        .spawn((
            RootUINode,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let inventory_ui = commands
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
                    padding: UiRect::all(Val::Px(8.)),
                    row_gap: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let top_inventory_ui = commands
        .spawn((
            TopInventoryUI,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let bottom_inventory_ui = commands
        .spawn((
            BottomInventoryUI,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect {
                        right: Val::Px(1.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let bottom_right_ui = commands
        .spawn((
            BottomRightUI,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let bottom_center_ui = commands
        .spawn((
            BottomCenterUI,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let bottom_left_ui = commands
        .spawn((
            BottomLeftUI,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands.entity(bottom_inventory_ui).push_children(&[
        bottom_left_ui,
        bottom_center_ui,
        bottom_right_ui,
    ]);
    commands
        .entity(inventory_ui)
        .push_children(&[top_inventory_ui, bottom_inventory_ui]);
    commands.entity(root_node).add_child(inventory_ui);
}

fn spawn_start_game_button(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    bottom_center_ui_q: Query<Entity, With<BottomCenterUI>>,
) {
    let start_game_button = commands
        .spawn((
            StartGameButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(65.),
                    height: Val::Px(16.),
                    padding: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(6.),
                        ..default()
                    },
                    ..default()
                },
                image: game_sprites.start_game_button.clone().into(),
                ..default()
            },
        ))
        .id();

    let button_text = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Start Game",
                TextStyle {
                    color: FONT_COLOR,
                    font_size: 7.,
                    font: game_fonts.font.clone(),
                },
            ),
            ..default()
        })
        .id();

    commands.entity(start_game_button).add_child(button_text);
    commands
        .entity(bottom_center_ui_q.single())
        .add_child(start_game_button);
}

fn destroy_start_button(mut commands: Commands, buttons_q: Query<Entity, With<StartGameButton>>) {
    for button in buttons_q.iter() {
        commands.entity(button).despawn_recursive();
    }
}

fn start_button_system(
    mut interaction_q: Query<(&Interaction, &mut UiImage), With<StartGameButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    game_sprites: Res<GameSprites>,
) {
    let (interaction, mut image) = interaction_q.single_mut();
    match *interaction {
        Interaction::Pressed => app_state.set(AppState::OrganizeInventory),
        Interaction::Hovered => image.texture = game_sprites.start_game_button_hover.clone(),
        Interaction::None => image.texture = game_sprites.start_game_button.clone(),
    };
}

fn spawn_restart_game_button(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    bottom_center_ui_q: Query<Entity, With<BottomCenterUI>>,
) {
    let restart_game_button = commands
        .spawn((
            RestartGameButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(65.),
                    height: Val::Px(16.),
                    padding: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(6.),
                        ..default()
                    },
                    ..default()
                },
                image: game_sprites.restart_game_button.clone().into(),
                ..default()
            },
        ))
        .id();

    let button_text = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Restart",
                TextStyle {
                    color: FONT_COLOR,
                    font_size: 7.,
                    font: game_fonts.font.clone(),
                },
            ),
            ..default()
        })
        .id();

    commands.entity(restart_game_button).add_child(button_text);
    commands
        .entity(bottom_center_ui_q.single())
        .add_child(restart_game_button);
}

fn destroy_restart_button(
    mut commands: Commands,
    buttons_q: Query<Entity, With<RestartGameButton>>,
) {
    for button in buttons_q.iter() {
        commands.entity(button).despawn_recursive();
    }
}

fn restart_button_system(
    mut interaction_q: Query<(&Interaction, &mut UiImage), With<RestartGameButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    game_sprites: Res<GameSprites>,
) {
    let (interaction, mut image) = interaction_q.single_mut();
    match *interaction {
        Interaction::Pressed => app_state.set(AppState::GameStart),
        Interaction::Hovered => image.texture = game_sprites.restart_game_button_hover.clone(),
        Interaction::None => image.texture = game_sprites.restart_game_button.clone(),
    };
}
