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

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AbilityModifier(i32);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Damage {
    base: i32,
    modifiers: AbilityModifier,
}

impl Damage {
    pub fn damage(&self) -> i32 {
        self.base + self.modifiers.0
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Jolly {
    base: i32,
    modifiers: AbilityModifier,
}

impl Jolly {
    pub fn jolly(&self) -> i32 {
        self.base + self.modifiers.0
    }
}
