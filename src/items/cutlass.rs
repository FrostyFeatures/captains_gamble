use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::common::Name;

use super::{
    abilities::{Damage, Jolly, SeaLegs, Squiffy},
    attributes::Pointy,
    Item,
};

const IRON_CUTLASS_BASE_DAMAGE: i32 = 3;

const BLESSED_CUTLASS_BASE_DAMAGE: i32 = 2;
const BLESSED_CUTLASS_BASE_JOLLY: i32 = 2;

const CURSED_CUTLASS_BASE_DAMAGE: i32 = 5;
const CURSED_CUTLASS_BASE_SQUIFFY: i32 = 1;

const CUTLASS_BASE_SEA_LEGS: i32 = 1;

pub enum CutlassType {
    Iron,
    Blessed,
    Cursed,
}

impl CutlassType {
    fn icon_id(&self) -> usize {
        match self {
            CutlassType::Iron => 4,
            CutlassType::Blessed => 5,
            CutlassType::Cursed => 6,
        }
    }
}

#[derive(Component)]
pub struct Cutlass {
    pub r#type: CutlassType,
}

impl Item for Cutlass {
    fn icon_id(&self) -> usize {
        self.r#type.icon_id()
    }

    fn add_bundle(&self, entity_commands: &mut EntityCommands) {
        match self.r#type {
            CutlassType::Iron => entity_commands.insert(IronCutlassBundle::default()),
            CutlassType::Blessed => entity_commands.insert(BlessedCutlassBundle::default()),
            CutlassType::Cursed => entity_commands.insert(CursedCutlassBundle::default()),
        };
    }
}
#[derive(Bundle)]
struct IronCutlassBundle {
    cutlass: Cutlass,
    name: Name,
    damage: Damage,
    sea_legs: SeaLegs,
    pointy: Pointy,
}

impl Default for IronCutlassBundle {
    fn default() -> Self {
        Self {
            cutlass: Cutlass {
                r#type: CutlassType::Iron,
            },
            name: Name("Blessed Cutlass".to_string()),
            damage: Damage {
                base: IRON_CUTLASS_BASE_DAMAGE,
                ..default()
            },
            sea_legs: SeaLegs {
                base: CUTLASS_BASE_SEA_LEGS,
                ..default()
            },
            pointy: Pointy,
        }
    }
}

#[derive(Bundle)]
struct BlessedCutlassBundle {
    cutlass: Cutlass,
    name: Name,
    damage: Damage,
    jolly: Jolly,
    sea_legs: SeaLegs,
    pointy: Pointy,
}

impl Default for BlessedCutlassBundle {
    fn default() -> Self {
        Self {
            cutlass: Cutlass {
                r#type: CutlassType::Blessed,
            },
            name: Name("Blessed Cutlass".to_string()),
            damage: Damage {
                base: BLESSED_CUTLASS_BASE_DAMAGE,
                ..default()
            },
            jolly: Jolly {
                base: BLESSED_CUTLASS_BASE_JOLLY,
                ..default()
            },
            sea_legs: SeaLegs {
                base: CUTLASS_BASE_SEA_LEGS,
                ..default()
            },
            pointy: Pointy,
        }
    }
}

#[derive(Bundle)]
struct CursedCutlassBundle {
    cutlass: Cutlass,
    name: Name,
    damage: Damage,
    squiffy: Squiffy,
    sea_legs: SeaLegs,
    pointy: Pointy,
}

impl Default for CursedCutlassBundle {
    fn default() -> Self {
        Self {
            cutlass: Cutlass {
                r#type: CutlassType::Cursed,
            },
            name: Name("Cursed Cutlass".to_string()),
            damage: Damage {
                base: CURSED_CUTLASS_BASE_DAMAGE,
                ..default()
            },
            squiffy: Squiffy {
                base: CURSED_CUTLASS_BASE_SQUIFFY,
                ..default()
            },
            sea_legs: SeaLegs {
                base: CUTLASS_BASE_SEA_LEGS,
                ..default()
            },
            pointy: Pointy,
        }
    }
}
