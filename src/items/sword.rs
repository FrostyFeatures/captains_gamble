use bevy::prelude::*;

use super::Item;

pub enum SwordType {
    Wooden,
    Iron,
    Magic,
}

#[derive(Component)]
pub struct Sword {
    pub r#type: SwordType,
}

impl Item for Sword {
    fn icon_id(&self) -> usize {
        match self.r#type {
            SwordType::Wooden => 0,
            SwordType::Iron => 1,
            SwordType::Magic => 2,
        }
    }
}
