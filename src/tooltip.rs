use bevy::{prelude::*, ui::RelativeCursorPosition, window::PrimaryWindow};

use crate::{
    assets::GameFonts,
    items::{Damage, Jolly},
    AppState,
};

const FONT_SIZE: f32 = 4.;
const FONT_COLOR: Color = Color::WHITE;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Damage>();
        app.register_component_as::<dyn TooltipComponent, Jolly>();

        app.add_systems(
            Update,
            (spawn_tooltips, update_tooltip_positions, destroy_tooltips)
                .chain()
                .run_if(not(in_state(AppState::LoadingAssets))),
        );
    }
}

#[bevy_trait_query::queryable]
pub trait TooltipComponent {
    fn get_tooltip_section(&self) -> TooltipSection;
}

#[derive(Component, Default, Debug)]
pub enum Tooltipable {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Debug)]
pub struct TooltipSection(pub String);

#[derive(Component, Debug)]
struct Tooltip(Vec<TooltipSection>);

#[derive(Component)]
struct TooltipRoot;

impl Tooltip {
    fn spawn(&self, commands: &mut Commands, game_fonts: &GameFonts) {
        commands
            .spawn((
                TooltipRoot,
                NodeBundle {
                    z_index: ZIndex::Global(i32::MAX),
                    background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(2.)),
                        justify_content: JustifyContent::Start,
                        row_gap: Val::Px(2.),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|root| {
                for text_section in self.0.iter() {
                    root.spawn(TextBundle {
                        text: Text::from_section(
                            text_section.0.clone(),
                            TextStyle {
                                color: FONT_COLOR,
                                font_size: FONT_SIZE,
                                font: game_fonts.font.clone(),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                }
            });
    }
}

fn spawn_tooltips(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    tooltipable_q: Query<
        (
            Entity,
            &dyn TooltipComponent,
            &RelativeCursorPosition,
            &Tooltipable,
        ),
        Without<Tooltip>,
    >,
) {
    for (entity, tooltip_components, relative_cursor_postition, tooltipable) in tooltipable_q.iter()
    {
        if let Tooltipable::Disabled = tooltipable {
            continue;
        }

        if !relative_cursor_postition.mouse_over() {
            continue;
        }
        let tooltip_sections = tooltip_components
            .iter()
            .map(|tc| tc.get_tooltip_section())
            .collect();
        let tooltip = Tooltip(tooltip_sections);
        tooltip.spawn(&mut commands, &game_fonts);
        commands.get_entity(entity).map(|mut ec| {
            ec.insert(tooltip);
        });
    }
}

fn update_tooltip_positions(
    mut tooltip_root_q: Query<&mut Style, With<TooltipRoot>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(position) = windows_q.single().cursor_position() else {
        return;
    };

    for mut style in tooltip_root_q.iter_mut() {
        style.left = Val::Px(position.x);
        style.top = Val::Px(position.y);
    }
}

fn destroy_tooltips(
    mut commands: Commands,
    tooltipable_q: Query<(Entity, &Tooltipable, &RelativeCursorPosition), With<Tooltip>>,
    tooltip_q: Query<Entity, With<TooltipRoot>>,
) {
    for (entity, tooltipable, relative_cursor_position) in tooltipable_q.iter() {
        if match tooltipable {
            Tooltipable::Disabled => true,
            Tooltipable::Enabled => !relative_cursor_position.mouse_over(),
        } {
            commands.entity(entity).remove::<Tooltip>();
            for tooltip in tooltip_q.iter() {
                commands.get_entity(tooltip).map(|ec| {
                    ec.despawn_recursive();
                });
            }
        }
    }
}
