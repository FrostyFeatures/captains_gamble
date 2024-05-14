use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::items::{Damage, Jolly};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Damage>();
        app.register_component_as::<dyn TooltipComponent, Jolly>();

        app.add_systems(Update, (spawn_tooltips, destroy_tooltips));
    }
}

#[bevy_trait_query::queryable]
pub trait TooltipComponent {
    fn get_tooltip_section(&self) -> TooltipSection;
}

#[derive(Component, Default)]
pub struct Tooltipable;

pub struct TooltipSection(pub String);

#[derive(Component)]
struct Tooltip(Vec<TooltipSection>);

fn spawn_tooltips(
    mut commands: Commands,
    tooltipable_q: Query<
        (Entity, &dyn TooltipComponent, &RelativeCursorPosition),
        (With<Tooltipable>, Without<Tooltip>),
    >,
) {
    for (entity, tooltip_components, relative_cursor_postition) in tooltipable_q.iter() {
        if !relative_cursor_postition.mouse_over() {
            continue;
        }
        let tooltip_sections = tooltip_components
            .iter()
            .map(|tc| tc.get_tooltip_section())
            .collect();
        let tooltip = Tooltip(tooltip_sections);
        commands.get_entity(entity).map(|mut ec| {
            println!("Adding tooltip for {:?}", entity);
            ec.insert(tooltip);
        });
    }
}

fn destroy_tooltips(
    mut commands: Commands,
    tooltipable_q: Query<(Entity, &RelativeCursorPosition), With<Tooltip>>,
) {
    for (entity, relative_cursor_position) in tooltipable_q.iter() {
        if relative_cursor_position.mouse_over() {
            continue;
        }

        commands.get_entity(entity).map(|mut ec| {
            println!("Destroying tooltip for {:?}", entity);
            ec.remove::<Tooltip>();
        });
    }
}
