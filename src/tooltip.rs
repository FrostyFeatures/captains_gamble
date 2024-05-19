use bevy::{prelude::*, ui::RelativeCursorPosition, window::PrimaryWindow};

use crate::{
    assets::{GameFonts, GameMaterials},
    common::Name,
    items::{
        abilities::{Cursed, Damage, Hearties, Heave, Jolly, SeaLegs, Swashbuckle, Vitality},
        attributes::{Cannonball, Flintlock, Pellets, Pointy},
        Consumable, Rarity,
    },
    AppState,
};

const HEADER_SIZE: f32 = 6.;
const BODY_SIZE: f32 = 4.;
const FOOTER_SIZE: f32 = 4.;

const HEADER_FONT_COLOR: Color = Color::WHITE;
const BODY_FONT_COLOR: Color = Color::WHITE;
const FOOTER_FONT_COLOR: Color = Color::GRAY;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Damage>();
        app.register_component_as::<dyn TooltipComponent, Hearties>();
        app.register_component_as::<dyn TooltipComponent, Cursed>();
        app.register_component_as::<dyn TooltipComponent, Heave>();
        app.register_component_as::<dyn TooltipComponent, SeaLegs>();
        app.register_component_as::<dyn TooltipComponent, Swashbuckle>();
        app.register_component_as::<dyn TooltipComponent, Jolly>();
        app.register_component_as::<dyn TooltipComponent, Vitality>();

        app.register_component_as::<dyn TooltipComponent, Pointy>();
        app.register_component_as::<dyn TooltipComponent, Flintlock>();
        app.register_component_as::<dyn TooltipComponent, Pellets>();
        app.register_component_as::<dyn TooltipComponent, Cannonball>();

        app.register_component_as::<dyn TooltipComponent, Name>();
        app.register_component_as::<dyn TooltipComponent, Rarity>();
        app.register_component_as::<dyn TooltipComponent, Consumable>();

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

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum TooltipSectionIndex {
    Header,
    Body,
    Footer,
}

impl TooltipSectionIndex {
    fn color(&self) -> Color {
        match self {
            TooltipSectionIndex::Header => HEADER_FONT_COLOR,
            TooltipSectionIndex::Body => BODY_FONT_COLOR,
            TooltipSectionIndex::Footer => FOOTER_FONT_COLOR,
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            TooltipSectionIndex::Header => HEADER_SIZE,
            TooltipSectionIndex::Body => BODY_SIZE,
            TooltipSectionIndex::Footer => FOOTER_SIZE,
        }
    }
}

#[derive(Debug)]
pub struct TooltipSection {
    pub text: String,
    pub index: TooltipSectionIndex,
    pub color: Color,
}

impl TooltipSection {
    pub fn default_color(text: String, index: TooltipSectionIndex) -> Self {
        let color = index.color();
        Self { text, index, color }
    }
}

#[derive(Component, Debug)]
struct Tooltip(Vec<TooltipSection>);

#[derive(Component)]
struct TooltipRoot;

impl Tooltip {
    fn spawn(
        &self,
        commands: &mut Commands,
        game_materials: &GameMaterials,
        game_fonts: &GameFonts,
    ) {
        commands
            .spawn((
                TooltipRoot,
                MaterialNodeBundle {
                    z_index: ZIndex::Global(i32::MAX),
                    // background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(2.)),
                        justify_content: JustifyContent::Start,
                        row_gap: Val::Px(2.),
                        ..default()
                    },
                    material: game_materials.text_bg.clone(),
                    ..default()
                },
            ))
            .with_children(|root| {
                for text_section in self.0.iter() {
                    root.spawn(TextBundle {
                        text: Text::from_section(
                            text_section.text.clone(),
                            TextStyle {
                                color: text_section.color,
                                font_size: text_section.index.font_size(),
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
    game_materials: Res<GameMaterials>,
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
        let mut tooltip_sections: Vec<TooltipSection> = tooltip_components
            .iter()
            .map(|tc| tc.get_tooltip_section())
            .collect();
        tooltip_sections.sort_by(|a, b| a.index.cmp(&b.index));
        let tooltip = Tooltip(tooltip_sections);
        tooltip.spawn(&mut commands, &game_materials, &game_fonts);
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
        style.left = Val::Px(position.x.floor());
        style.top = Val::Px(position.y.floor());
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
