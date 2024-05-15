use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{Damage, Item, Jolly, Squiffy};

const WOODEN_SWORD_BASE_DAMAGE: i32 = 3;
const IRON_SWORD_BASE_DAMAGE: i32 = 5;
const BLESSED_SWORD_BASE_DAMAGE: i32 = 4;
const CURSED_SWORD_BASE_DAMAGE: i32 = 8;
const BLESSED_SWORD_BASE_JOLLY: i32 = 2;
const CURSED_SWORD_BASE_SQUIFFY: i32 = 2;

pub enum SwordType {
    Wooden,
    Iron,
    Blessed,
    Cursed,
}

impl SwordType {
    fn icon_id(&self) -> usize {
        match self {
            SwordType::Wooden => 0,
            SwordType::Iron => 1,
            SwordType::Blessed => 2,
            SwordType::Cursed => 3,
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

    fn add_bundle(&self, entity_commands: &mut EntityCommands) {
        match self.r#type {
            SwordType::Wooden => entity_commands.insert(WoodenSwordBundle::default()),
            SwordType::Iron => entity_commands.insert(IronSwordBundle::default()),
            SwordType::Blessed => entity_commands.insert(BlessedSwordBundle::default()),
            SwordType::Cursed => entity_commands.insert(CursedSwordBundle::default()),
        };
    }
}

#[derive(Bundle)]
struct WoodenSwordBundle {
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
struct IronSwordBundle {
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
struct BlessedSwordBundle {
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

#[derive(Bundle)]
struct CursedSwordBundle {
    sword: Sword,
    damage: Damage,
    squiffy: Squiffy,
}

impl Default for CursedSwordBundle {
    fn default() -> Self {
        Self {
            sword: Sword {
                r#type: SwordType::Cursed,
            },
            damage: Damage {
                base: CURSED_SWORD_BASE_DAMAGE,
                ..default()
            },
            squiffy: Squiffy {
                base: CURSED_SWORD_BASE_SQUIFFY,
                ..default()
            },
        }
    }
}
