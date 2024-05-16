use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::common::Name;

use super::{
    abilities::{AbilityTarget, Heave},
    Item,
};

#[derive(Component)]
pub struct Flag;

impl Item for Flag {
    fn icon_id(&self) -> usize {
        31
    }

    fn add_bundle(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(FlagBundle::default());
    }
}

#[derive(Bundle)]
struct FlagBundle {
    flag: Flag,
    name: Name,
    heave: Heave,
}

impl Default for FlagBundle {
    fn default() -> Self {
        Self {
            flag: Flag,
            name: Name("Flag".to_string()),
            heave: Heave {
                base: 1,
                target: AbilityTarget::All,
                ..default()
            },
        }
    }
}
