use bevy::prelude::*;

use crate::common::Hp;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub hp: Hp,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            hp: Hp::new(10),
        }
    }
}
