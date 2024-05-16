use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::common::Name;

use super::{
    abilities::{AbilityTarget, Heave, TargetFilter},
    attributes::POINTY,
    Item,
};

#[derive(Component)]
pub struct Spyglass;

impl Item for Spyglass {
    fn icon_id(&self) -> usize {
        23
    }

    fn add_bundle(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(SpyglassBundle::default());
    }
}

#[derive(Bundle)]
struct SpyglassBundle {
    spyglass: Spyglass,
    name: Name,
    heave: Heave,
}

impl Default for SpyglassBundle {
    fn default() -> Self {
        Self {
            spyglass: Spyglass,
            name: Name("Spyglass".to_string()),
            heave: Heave {
                base: 3,
                target: AbilityTarget {
                    filter: TargetFilter::Next,
                    attribute: POINTY.to_string(),
                },
                ..default()
            },
        }
    }
}
