use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::items::{
    abilities::AbilityPlugin, attributes::AttributePlugin, flag::Flag, spyglass::Spyglass,
    sword::Sword,
};
mod abilities;
pub mod attributes;
pub mod flag;
pub mod spyglass;
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>();
        app.register_component_as::<dyn Item, Flag>();
        app.register_component_as::<dyn Item, Spyglass>();

        app.add_plugins((AbilityPlugin, AttributePlugin));
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
    fn add_bundle(&self, entity_commands: &mut EntityCommands);
}
