use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    battle::UseItem,
    common::Hp,
    enemy::Enemy,
    items::sword::Sword,
    log::LogMessageEvent,
    player::Player,
    tooltip::{TooltipComponent, TooltipSection},
    AppState,
};
pub mod sword;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Item, Sword>().add_systems(
            Update,
            (handle_damage_use, handle_jolly_use)
                .chain()
                .run_if(in_state(AppState::Battling)),
        );
    }
}

#[bevy_trait_query::queryable]
pub trait Item {
    fn icon_id(&self) -> usize;
    fn add_bundle(&self, entity_commands: &mut EntityCommands);
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

impl TooltipComponent for Damage {
    fn get_tooltip_section(&self) -> TooltipSection {
        TooltipSection(format!("Damage {}", self.damage()))
    }
}

fn handle_damage_use(
    mut log_message_er: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut enemy_hp_q: Query<&mut Hp, With<Enemy>>,
    damage_q: Query<&Damage>,
) {
    let Ok(mut enemy_hp) = enemy_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(damage) = damage_q.get(item_e.0) else {
            continue;
        };
        log_message_er.send(LogMessageEvent(format!(
            "Dealt {} damage!",
            damage.damage()
        )));
        enemy_hp.decrease(damage.damage());
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

impl TooltipComponent for Jolly {
    fn get_tooltip_section(&self) -> TooltipSection {
        TooltipSection(format!("Jolly {}", self.base))
    }
}

fn handle_jolly_use(
    mut log_message_er: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    jolly_q: Query<&Jolly>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(jolly) = jolly_q.get(item_e.0) else {
            continue;
        };
        log_message_er.send(LogMessageEvent(format!("Healed {} health!", jolly.jolly())));
        player_hp.increase(jolly.jolly());
    }
}
