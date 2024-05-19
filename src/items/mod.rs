use bevy::{ecs::system::EntityCommands, prelude::*, ui::RelativeCursorPosition};
use rand::Rng;

use crate::{
    assets::GameSprites,
    common::Name,
    inventory::Draggable,
    items::{abilities::AbilityPlugin, attributes::AttributePlugin},
    tooltip::{TooltipComponent, TooltipSection, TooltipSectionIndex, Tooltipable},
};

use self::{
    abilities::{
        AbilityTarget, Cursed, Damage, Hearties, Heave, Jolly, SeaLegs, Swashbuckle, TargetFilter,
    },
    attributes::{Cannonball, Flintlock, Pellets, Pointy, CANNONBALL, FLINTLOCK, PELLETS, POINTY},
};
pub mod abilities;
pub mod attributes;

const MUNDANGE_ITEMS: &[ItemType] = &[
    ItemType::BagOfPellets,
    ItemType::Orange,
    ItemType::Grog,
    ItemType::WoodenSword,
];

const SCARCE_ITEMS: &[ItemType] = &[
    ItemType::IronSword,
    ItemType::IronCutlass,
    ItemType::Flag,
    ItemType::Spyglass,
    ItemType::Blunderbuss,
];

const PRECIOUS_ITEMS: &[ItemType] = &[
    ItemType::BlessedSword,
    ItemType::BlessedCutlass,
    ItemType::CursedSword,
    ItemType::CursedCutlass,
];

const MYTHIC_ITEMS: &[ItemType] = &[
    ItemType::JewelOfTheSea,
    ItemType::JewelOfTheEarth,
    ItemType::JewelOfLife,
    ItemType::CursedJewel,
];

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AbilityPlugin, AttributePlugin));
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub enum Rarity {
    Mundane,
    Scarce,
    Precious,
    Mythic,
}

impl Rarity {
    fn rand_item(&self, rng: &mut crate::rng::Rng) -> ItemType {
        match self {
            Rarity::Mundane => MUNDANGE_ITEMS[rng.0.gen_range(0..MUNDANGE_ITEMS.len())],
            Rarity::Scarce => SCARCE_ITEMS[rng.0.gen_range(0..SCARCE_ITEMS.len())],
            Rarity::Precious => PRECIOUS_ITEMS[rng.0.gen_range(0..PRECIOUS_ITEMS.len())],
            Rarity::Mythic => MYTHIC_ITEMS[rng.0.gen_range(0..MYTHIC_ITEMS.len())],
        }
    }

    fn name(&self) -> String {
        match self {
            Rarity::Mundane => "Mundane".to_string(),
            Rarity::Scarce => "Scarce".to_string(),
            Rarity::Precious => "Precious".to_string(),
            Rarity::Mythic => "Mythic".to_string(),
        }
    }
}

impl From<ItemType> for Rarity {
    fn from(value: ItemType) -> Self {
        for item in MUNDANGE_ITEMS {
            if value == *item {
                return Self::Mundane;
            }
        }
        for item in SCARCE_ITEMS {
            if value == *item {
                return Self::Scarce;
            }
        }
        for item in PRECIOUS_ITEMS {
            if value == *item {
                return Self::Precious;
            }
        }
        for item in MYTHIC_ITEMS {
            if value == *item {
                return Self::Mythic;
            }
        }
        panic!("Item has no rarity assigned");
    }
}

impl TooltipComponent for Rarity {
    fn get_tooltip_section(&self) -> TooltipSection {
        TooltipSection {
            text: self.name(),
            index: TooltipSectionIndex::Footer,
            color: match self {
                Rarity::Mundane => Color::GRAY,
                Rarity::Scarce => Color::BLUE,
                Rarity::Precious => Color::ORANGE_RED,
                Rarity::Mythic => Color::PURPLE,
            },
        }
    }
}

#[derive(Component, PartialEq, Clone, Copy, Debug)]
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
    JewelOfTheSea,   // + sea legs
    JewelOfLife,     // + hearties
    JewelOfTheEarth, // + ??
    CursedJewel,     // Cursed, + Damage
    Orange,
    BagOfBeans,
    MurkyBroth,
    Blunderbuss,
    BagOfPellets,
    Cannon,
    Cannonball,
    ChainShot,
    CursedVial,
    VialOfLife,
    VialOfTheSea,
    VialOfTheEarth,
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
            ItemType::JewelOfTheSea => 36,
            ItemType::JewelOfLife => 35,
            ItemType::JewelOfTheEarth => 37,
            ItemType::CursedJewel => 34,
            ItemType::Orange => 16,
            ItemType::BagOfBeans => 17,
            ItemType::MurkyBroth => 18,
            ItemType::Blunderbuss => 8,
            ItemType::BagOfPellets => 9,
            ItemType::Cannon => 13,
            ItemType::Cannonball => 14,
            ItemType::ChainShot => 15,
            ItemType::CursedVial => 26,
            ItemType::VialOfLife => 27,
            ItemType::VialOfTheSea => 28,
            ItemType::VialOfTheEarth => 29,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ItemType::Grog => "Grog".to_string(),
            ItemType::WoodenSword => "Wooden Sword".to_string(),
            ItemType::IronSword => "Iron Sword".to_string(),
            ItemType::BlessedSword => "Blessed Sword".to_string(),
            ItemType::CursedSword => "Cursed Sword".to_string(),
            ItemType::IronCutlass => "Iron Cutlass".to_string(),
            ItemType::BlessedCutlass => "Blessed Cutlass".to_string(),
            ItemType::CursedCutlass => "Cursed Cutlass".to_string(),
            ItemType::Flag => "Flag".to_string(),
            ItemType::Spyglass => "Spyglass".to_string(),
            ItemType::JewelOfTheSea => "Jewel of the Sea".to_string(),
            ItemType::JewelOfLife => "Jewel of Life".to_string(),
            ItemType::JewelOfTheEarth => "Jewel of the Earth".to_string(),
            ItemType::CursedJewel => "Cursed Jewel".to_string(),
            ItemType::Orange => "Orange".to_string(),
            ItemType::BagOfBeans => "Bag Of Beans".to_string(),
            ItemType::MurkyBroth => "Murky Broth".to_string(),
            ItemType::Blunderbuss => "Blunderbuss".to_string(),
            ItemType::BagOfPellets => "Bag Of Pellets".to_string(),
            ItemType::Cannon => "Cannon".to_string(),
            ItemType::Cannonball => "Cannonball".to_string(),
            ItemType::ChainShot => "Chain Shot".to_string(),
            ItemType::CursedVial => "Cursed Vial".to_string(),
            ItemType::VialOfLife => "Vial Of Life".to_string(),
            ItemType::VialOfTheSea => "Vial Of The Sea".to_string(),
            ItemType::VialOfTheEarth => "Vial Of The Earth".to_string(),
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
            Rarity::from(*self),
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
                entity_commands.insert((Damage::new(4), Hearties::new(2), Pointy))
            }
            ItemType::CursedSword => {
                entity_commands.insert((Damage::new(8), Cursed::new(2), Pointy))
            }
            ItemType::IronCutlass => {
                entity_commands.insert((Damage::new(3), SeaLegs::new(1), Pointy))
            }
            ItemType::BlessedCutlass => {
                entity_commands.insert((Damage::new(2), SeaLegs::new(1), Hearties::new(1), Pointy))
            }
            ItemType::CursedCutlass => {
                entity_commands.insert((Damage::new(5), SeaLegs::new(2), Cursed::new(2), Pointy))
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
            ItemType::JewelOfTheSea => entity_commands.insert((Swashbuckle::new(
                1,
                AbilityTarget::with_all_attributes(TargetFilter::All),
            ),)),
            ItemType::JewelOfLife => entity_commands.insert((Jolly::new(
                1,
                AbilityTarget::with_all_attributes(TargetFilter::All),
            ),)),
            ItemType::JewelOfTheEarth => todo!(),
            ItemType::CursedJewel => entity_commands.insert((
                Cursed::new(2),
                Heave::new(1, AbilityTarget::with_all_attributes(TargetFilter::All)),
            )),
            ItemType::Orange => entity_commands.insert((Hearties::new(7), Consumable(1))),
            ItemType::BagOfBeans => entity_commands.insert((Hearties::new(4), Consumable(2))),
            ItemType::MurkyBroth => {
                entity_commands.insert((Hearties::new(3), SeaLegs::new(1), Consumable(2)))
            }
            ItemType::Blunderbuss => {
                entity_commands.insert((Flintlock::empty(PELLETS.to_string(), 8), Damage::new(15)))
            }
            ItemType::BagOfPellets => entity_commands.insert((
                Pellets {
                    load_amount: 2,
                    target: AbilityTarget {
                        filter: TargetFilter::AllNext,
                        attribute: FLINTLOCK.to_string(),
                    },
                },
                Consumable(6),
            )),
            ItemType::Cannon => entity_commands
                .insert((Flintlock::empty(CANNONBALL.to_string(), 27), Damage::new(5))),
            ItemType::Cannonball => entity_commands.insert((
                Cannonball {
                    load_amount: 1,
                    target: AbilityTarget {
                        filter: TargetFilter::Next(1),
                        attribute: FLINTLOCK.to_string(),
                    },
                },
                Consumable(1),
            )),
            ItemType::ChainShot => entity_commands.insert((
                Cannonball {
                    load_amount: 1,
                    target: AbilityTarget {
                        filter: TargetFilter::Prev(2),
                        attribute: FLINTLOCK.to_string(),
                    },
                },
                Consumable(1),
            )),
            ItemType::CursedVial => entity_commands.insert(()),
            ItemType::VialOfLife => entity_commands.insert(()),
            ItemType::VialOfTheSea => entity_commands.insert(()),
            ItemType::VialOfTheEarth => entity_commands.insert(()),
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
        TooltipSection::default_color(format!("Consumable {}", self.0), TooltipSectionIndex::Body)
    }
}
