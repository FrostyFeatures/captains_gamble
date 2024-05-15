use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::items::{abilities::AbilityPlugin, attributes::AttributePlugin, sword::Sword};
mod abilities;
mod attributes;
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>();

        app.add_plugins((AbilityPlugin, AttributePlugin));
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
    fn add_bundle(&self, entity_commands: &mut EntityCommands);
}
