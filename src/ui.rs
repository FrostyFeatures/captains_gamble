use bevy::prelude::*;

use crate::AppState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitGame), setup_root_node);
    }
}

#[derive(Component)]
pub struct RootUINode;

fn setup_root_node(mut commands: Commands) {
    commands.spawn((
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
    ));
}
