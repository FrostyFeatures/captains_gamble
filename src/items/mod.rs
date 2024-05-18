use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    items::{
        abilities::AbilityPlugin, attributes::AttributePlugin, cutlass::Cutlass, flag::Flag,
        grog::Grog, spyglass::Spyglass, sword::Sword,
    },
    tooltip::{TooltipComponent, TooltipSection},
};
pub mod abilities;
pub mod attributes;
pub mod cutlass;
pub mod flag;
pub mod grog;
pub mod spyglass;
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>();
        app.register_component_as::<dyn Item, Cutlass>();
        app.register_component_as::<dyn Item, Flag>();
        app.register_component_as::<dyn Item, Spyglass>();
        app.register_component_as::<dyn Item, Grog>();

        app.add_plugins((AbilityPlugin, AttributePlugin));
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
    fn add_bundle(&self, entity_commands: &mut EntityCommands);
}

#[derive(Component)]
pub struct Consumable(pub i32);

impl TooltipComponent for Consumable {
    fn get_tooltip_section(&self) -> TooltipSection {
        TooltipSection {
            text: format!("Consumable {}", self.0),
            index: i32::MAX,
        }
    }
}
