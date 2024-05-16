use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::common::Name;

use super::{abilities::SeaLegs, Consumable, Item};

#[derive(Component)]
pub struct Grog;

impl Item for Grog {
    fn icon_id(&self) -> usize {
        33
    }

    fn add_bundle(&self, entity_commands: &mut EntityCommands) {
        entity_commands.insert(GrogBundle::default());
    }
}

#[derive(Bundle)]
struct GrogBundle {
    grog: Grog,
    name: Name,
    consumable: Consumable,
    sea_legs: SeaLegs,
}

impl Default for GrogBundle {
    fn default() -> Self {
        Self {
            grog: Grog,
            name: Name("Grog".to_string()),
            consumable: Consumable(5),
            sea_legs: SeaLegs {
                base: 3,
                ..default()
            },
        }
    }
}
