use std::collections::VecDeque;

use bevy::{prelude::*, utils::petgraph::visit::DfsEvent};
use bevy_trait_query::One;

use crate::{
    assets::GameSprites,
    dice::{DicePlugin, Die, NormalDie},
    items::{
        sword::{Sword, SwordType},
        Item, ItemPlugin,
    },
    AppState, Rng,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DicePlugin)
            .add_plugins(ItemPlugin)
            .init_resource::<Inventory>()
            .add_systems(OnEnter(AppState::Game), (setup_inventory, open_gui).chain());
    }
}

pub enum InventoryQueue {
    Locked,
    Active(VecDeque<Entity>),
}

#[derive(Resource)]
pub struct Inventory {
    pub queues: [InventoryQueue; 6],
    pub dice: Vec<Entity>,
}

impl Inventory {
    pub fn add_die(&mut self, entity: Entity) {
        self.dice.push(entity);
    }

    pub fn add_item(&mut self, entity: Entity, queue_index: usize) -> bool {
        if queue_index >= 6 {
            return false;
        }
        let InventoryQueue::Active(queue) = &mut self.queues[queue_index] else {
            return false;
        };
        queue.push_back(entity);
        return true;
    }

    pub fn queue_count(&self) -> usize {
        self.queues
            .iter()
            .filter(|q| matches!(q, InventoryQueue::Active(_)))
            .count()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            queues: [
                InventoryQueue::Active(VecDeque::new()),
                InventoryQueue::Active(VecDeque::new()),
                InventoryQueue::Active(VecDeque::new()),
                InventoryQueue::Locked,
                InventoryQueue::Locked,
                InventoryQueue::Locked,
            ],
            dice: Vec::new(),
        }
    }
}

fn setup_inventory(mut commands: Commands, mut inventory: ResMut<Inventory>) {
    // Starting inventory has 3 normal die
    inventory.add_die(NormalDie::spawn_new(&mut commands));
    inventory.add_die(NormalDie::spawn_new(&mut commands));
    inventory.add_die(NormalDie::spawn_new(&mut commands));

    inventory.add_item(
        commands
            .spawn(Sword {
                r#type: SwordType::Wooden,
            })
            .id(),
        0,
    );
    inventory.add_item(
        commands
            .spawn(Sword {
                r#type: SwordType::Iron,
            })
            .id(),
        1,
    );
    inventory.add_item(
        commands
            .spawn(Sword {
                r#type: SwordType::Magic,
            })
            .id(),
        2,
    );
    inventory.add_item(
        commands
            .spawn(Sword {
                r#type: SwordType::Magic,
            })
            .id(),
        2,
    );
    inventory.add_item(
        commands
            .spawn(Sword {
                r#type: SwordType::Magic,
            })
            .id(),
        2,
    );
}

fn open_gui(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    inventory: Res<Inventory>,
    items_q: Query<One<&dyn Item>>,
) {
    println!("{}", items_q.iter().len());
    commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(game_sprites.inventory_bg.clone()),
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        width: Val::Px(260.),
                        height: Val::Px(120.),
                        padding: UiRect {
                            right: Val::Px(8.),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for queue in inventory.queues.iter() {
                        spawn_inventory_queue(parent, &game_sprites, queue, &items_q);
                    }
                });
        });
}

fn spawn_inventory_queue(
    builder: &mut ChildBuilder,
    game_sprites: &GameSprites,
    queue: &InventoryQueue,
    items_q: &Query<One<&dyn Item>>,
) {
    builder
        .spawn(ImageBundle {
            image: UiImage::new(game_sprites.inventory_cloth.clone()),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect {
                    top: Val::Px(2.),
                    bottom: Val::Px(2.),
                    ..default()
                },
                row_gap: Val::Px(4.),
                width: Val::Px(27.),
                height: Val::Px(101.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let InventoryQueue::Active(entities) = queue else {
                return;
            };

            for entity in entities {
                let Ok(item) = items_q.get(*entity) else {
                    continue;
                };

                parent.spawn((AtlasImageBundle {
                    image: UiImage::new(game_sprites.items_tile_sheet.clone()),
                    texture_atlas: TextureAtlas {
                        layout: game_sprites.items_tile_layout.clone(),
                        index: item.icon_id(),
                    },
                    style: Style {
                        width: Val::Px(16.),
                        height: Val::Px(16.),
                        ..default()
                    },
                    ..default()
                },));
            }
        });
}

