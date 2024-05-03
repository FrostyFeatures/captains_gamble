use bevy::prelude::*;

use crate::items::sword::Sword;
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>();
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
}
