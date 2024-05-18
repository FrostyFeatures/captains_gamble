use bevy::{ecs::system::EntityCommands, prelude::*, ui::RelativeCursorPosition};

use crate::{
    assets::GameSprites,
    common::Name,
    inventory::Draggable,
    items::{
        abilities::AbilityPlugin, attributes::AttributePlugin, cutlass::Cutlass, flag::Flag,
        grog::Grog, spyglass::Spyglass, sword::Sword,
    },
    tooltip::{TooltipComponent, TooltipSection, Tooltipable},
};

use self::{
    abilities::{AbilityTarget, Damage, Heave, Jolly, SeaLegs, Squiffy, TargetFilter},
    attributes::{Pointy, POINTY},
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

#[derive(Component, Clone, Copy, Debug)]
pub enum ItemType {
    WoodenSword,
    IronSword,
    BlessedSword,
    CursedSword,
    IronCutlass,
    BlessedCutlass,
    CursedCutlass,
    Flag,
    Spyglass,
    Grog,
}

impl ItemType {
    pub fn image_index(&self) -> usize {
        match self {
            ItemType::WoodenSword => 0,
            ItemType::IronSword => 1,
            ItemType::BlessedSword => 2,
            ItemType::CursedSword => 3,
            ItemType::IronCutlass => 4,
            ItemType::BlessedCutlass => 5,
            ItemType::CursedCutlass => 6,
            ItemType::Flag => 31,
            ItemType::Spyglass => 23,
            ItemType::Grog => 33,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ItemType::WoodenSword => "Wooden Sword".to_string(),
            ItemType::IronSword => "Iron Sword".to_string(),
            ItemType::BlessedSword => "Blessed Sword".to_string(),
            ItemType::CursedSword => "Cursed Sword".to_string(),
            ItemType::IronCutlass => "Iron Cutlass".to_string(),
            ItemType::BlessedCutlass => "Blessed Cutlass".to_string(),
            ItemType::CursedCutlass => "Cursed Cutlass".to_string(),
            ItemType::Flag => "Flag".to_string(),
            ItemType::Spyglass => "Spyglass".to_string(),
            ItemType::Grog => "Grog".to_string(),
        }
    }

    pub fn spawn(&self, parent: &mut ChildBuilder, game_sprites: &GameSprites) {
        let bundle = (
            AtlasImageBundle {
                image: UiImage::new(game_sprites.items_tile_sheet.clone()),
                texture_atlas: TextureAtlas {
                    layout: game_sprites.items_tile_layout.clone(),
                    index: self.image_index(),
                },
                style: Style {
                    width: Val::Px(16.),
                    height: Val::Px(16.),
                    ..default()
                },
                ..default()
            },
            Name(self.name()),
            Draggable,
            RelativeCursorPosition::default(),
            Tooltipable::default(),
        );
        let entity_commands = parent.spawn(bundle);
        self.insert(entity_commands);
    }

    fn insert(&self, mut entity_commands: EntityCommands) {
        match self {
            ItemType::WoodenSword => entity_commands.insert((Damage::new(3), Pointy)),
            ItemType::IronSword => entity_commands.insert((Damage::new(5), Pointy)),
            ItemType::BlessedSword => {
                entity_commands.insert((Damage::new(4), Jolly::new(2), Pointy))
            }
            ItemType::CursedSword => {
                entity_commands.insert((Damage::new(8), Squiffy::new(2), Pointy))
            }
            ItemType::IronCutlass => {
                entity_commands.insert((Damage::new(3), SeaLegs::new(1), Pointy))
            }
            ItemType::BlessedCutlass => {
                entity_commands.insert((Damage::new(2), SeaLegs::new(1), Jolly::new(1), Pointy))
            }
            ItemType::CursedCutlass => {
                entity_commands.insert((Damage::new(5), SeaLegs::new(2), Squiffy::new(2), Pointy))
            }
            ItemType::Flag => entity_commands.insert((Heave::new(
                2,
                AbilityTarget {
                    filter: TargetFilter::Neighbours,
                    attribute: POINTY.to_string(),
                },
            ),)),
            ItemType::Spyglass => entity_commands.insert((Heave::new(
                1,
                AbilityTarget {
                    filter: TargetFilter::Next(3),
                    attribute: POINTY.to_string(),
                },
            ),)),
            ItemType::Grog => entity_commands.insert((SeaLegs::new(3), Consumable(3))),
        };
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
