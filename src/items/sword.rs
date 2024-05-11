use bevy::prelude::*;

use super::{Damage, Item, Jolly};

const WOODEN_SWORD_BASE_DAMAGE: i32 = 3;
const IRON_SWORD_BASE_DAMAGE: i32 = 5;
const BLESSED_SWORD_BASE_DAMAGE: i32 = 4;
const BLESSED_SWORD_BASE_JOLLY: i32 = 2;

pub enum SwordType {
    Wooden,
    Iron,
    Blessed,
}

impl SwordType {
    fn icon_id(&self) -> usize {
        match self {
            SwordType::Wooden => 0,
            SwordType::Iron => 1,
            SwordType::Blessed => 2,
        }
    }
}

#[derive(Component)]
pub struct Sword {
    pub r#type: SwordType,
}

impl Item for Sword {
    fn icon_id(&self) -> usize {
        self.r#type.icon_id()
    }
}

#[derive(Bundle)]
pub struct WoodenSwordBundle {
    sword: Sword,
    damage: Damage,
}

impl Default for WoodenSwordBundle {
    fn default() -> Self {
        Self {
            sword: Sword {
                r#type: SwordType::Wooden,
            },
            damage: Damage {
                base: WOODEN_SWORD_BASE_DAMAGE,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct IronSwordBundle {
    sword: Sword,
    damage: Damage,
}

impl Default for IronSwordBundle {
    fn default() -> Self {
        Self {
            sword: Sword {
                r#type: SwordType::Iron,
            },
            damage: Damage {
                base: IRON_SWORD_BASE_DAMAGE,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct BlessedSwordBundle {
    sword: Sword,
    damage: Damage,
    jolly: Jolly,
}

impl Default for BlessedSwordBundle {
    fn default() -> Self {
        Self {
            sword: Sword {
                r#type: SwordType::Blessed,
            },
            damage: Damage {
                base: BLESSED_SWORD_BASE_DAMAGE,
                ..default()
            },
            jolly: Jolly {
                base: BLESSED_SWORD_BASE_JOLLY,
                ..default()
            },
        }
    }
}
