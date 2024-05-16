use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    battle::UseItem,
    items::{
        abilities::AbilityPlugin, attributes::AttributePlugin, flag::Flag, grog::Grog,
        spyglass::Spyglass, sword::Sword,
    },
    AppState,
};
mod abilities;
pub mod attributes;
pub mod flag;
pub mod grog;
pub mod spyglass;
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>();
        app.register_component_as::<dyn Item, Flag>();
        app.register_component_as::<dyn Item, Spyglass>();
        app.register_component_as::<dyn Item, Grog>();

        app.add_plugins((AbilityPlugin, AttributePlugin));
        app.add_systems(
            Update,
            handle_consumable_use.run_if(in_state(AppState::Battling)),
        );
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
    fn add_bundle(&self, entity_commands: &mut EntityCommands);
}

#[derive(Component)]
pub struct Consumable(i32);

fn handle_consumable_use(
    mut use_item_er: EventReader<UseItem>,
    mut consumables_q: Query<&mut Consumable>,
) {
    for item_e in use_item_er.read() {
        let Ok(mut consumable) = consumables_q.get_mut(item_e.0) else {
            continue;
        };

        consumable.0 -= 1;
    }
}
