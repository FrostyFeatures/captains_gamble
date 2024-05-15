use bevy::{prelude::*, utils::HashMap};

use crate::{
    battle::UseItem,
    common::Hp,
    enemy::Enemy,
    inventory::InventoryScrollUI,
    log::LogMessageEvent,
    player::Player,
    tooltip::{TooltipComponent, TooltipSection},
    AppState,
};

use super::attributes::Pointy;

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Damage>();
        app.register_component_as::<dyn TooltipComponent, Jolly>();
        app.register_component_as::<dyn TooltipComponent, Squiffy>();
        app.register_component_as::<dyn TooltipComponent, Heave>();

        app.register_component_as::<dyn Ability, Damage>();
        app.register_component_as::<dyn Ability, Jolly>();
        app.register_component_as::<dyn Ability, Squiffy>();
        app.register_component_as::<dyn Ability, Heave>();

        app.add_systems(
            Update,
            (
                handle_damage_use,
                handle_jolly_use,
                handle_squiffy_use,
                handle_heave_use,
            )
                .chain()
                .run_if(in_state(AppState::Battling)),
        )
        .add_systems(OnExit(AppState::Battling), clear_ability_modifiers);
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AbilityModifier {
    pub amount: i32,
}

pub type AbilityModifiers = HashMap<Entity, AbilityModifier>;

#[bevy_trait_query::queryable]
pub trait Ability {
    fn name(&self) -> &'static str;
    fn base(&self) -> i32;
    fn modifiers(&self) -> &AbilityModifiers;
    fn modifiers_mut(&mut self) -> &mut AbilityModifiers;
    fn amount(&self) -> i32 {
        self.base()
            + self
                .modifiers()
                .iter()
                .fold(0, |sum, (_, m)| sum + m.amount)
    }
}

impl<T> TooltipComponent for T
where
    T: Ability + 'static,
{
    fn get_tooltip_section(&self) -> TooltipSection {
        let mut text = format!("{} {}", self.name(), self.base());
        for (_, modifier) in self.modifiers().iter() {
            if modifier.amount > 0 {
                text.push_str(format!("\n\t+{}", modifier.amount).as_str());
            } else if modifier.amount <= 0 {
                text.push_str(format!("\n\t{}", modifier.amount).as_str());
            }
        }
        TooltipSection { text, index: 1 }
    }
}

#[derive(Clone, Default, Copy, Debug)]
pub enum AbilityTarget {
    #[default]
    All,
    Next,
    Prev,
    AllNext,
    AllPrev,
}

impl AbilityTarget {
    fn get_targets<'a>(
        &self,
        index: usize,
        entity: Entity,
        list: impl IntoIterator<Item = &'a Entity>,
    ) -> Vec<Entity> {
        let iter = list.into_iter().enumerate().filter(|(_, c)| **c != entity);
        let targets: Vec<(usize, &Entity)> = match self {
            AbilityTarget::All => iter.collect(),
            AbilityTarget::AllNext => iter.filter(|(i, _)| *i > index).collect(),
            AbilityTarget::Next => iter.filter(|(i, _)| *i == index + 1).collect(),
            AbilityTarget::AllPrev => iter.filter(|(i, _)| *i < index).collect(),
            AbilityTarget::Prev => iter.filter(|(i, _)| *i == index - 1).collect(),
        };
        targets.iter().map(|(_, e)| **e).collect()
    }
}

fn clear_ability_modifiers(mut abilities_q: Query<&mut dyn Ability>) {
    for mut foo in abilities_q.iter_mut() {
        for mut bar in foo.iter_mut() {
            *bar.modifiers_mut() = AbilityModifiers::default();
        }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Damage {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl Ability for Damage {
    fn name(&self) -> &'static str {
        "Damage"
    }

    fn base(&self) -> i32 {
        self.base
    }

    fn modifiers(&self) -> &AbilityModifiers {
        &self.modifiers
    }

    fn modifiers_mut(&mut self) -> &mut AbilityModifiers {
        &mut self.modifiers
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
        let amount = damage.amount();
        log_message_er.send(LogMessageEvent(format!("Dealt {} damage!", amount)));
        enemy_hp.decrease(amount);
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Jolly {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl Ability for Jolly {
    fn name(&self) -> &'static str {
        "Jolly"
    }

    fn base(&self) -> i32 {
        self.base
    }

    fn modifiers(&self) -> &AbilityModifiers {
        &self.modifiers
    }

    fn modifiers_mut(&mut self) -> &mut AbilityModifiers {
        &mut self.modifiers
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
        let amount = jolly.amount();
        log_message_er.send(LogMessageEvent(format!("Healed {} health!", amount)));
        player_hp.increase(amount);
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Squiffy {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl Ability for Squiffy {
    fn name(&self) -> &'static str {
        "Squiffy"
    }

    fn base(&self) -> i32 {
        self.base
    }

    fn modifiers(&self) -> &AbilityModifiers {
        &self.modifiers
    }

    fn modifiers_mut(&mut self) -> &mut AbilityModifiers {
        &mut self.modifiers
    }
}

fn handle_squiffy_use(
    mut log_message_er: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut player_hp_q: Query<&mut Hp, With<Player>>,
    squiffy_q: Query<&Squiffy>,
) {
    let Ok(mut player_hp) = player_hp_q.get_single_mut() else {
        return;
    };
    for item_e in use_item_ev.read() {
        let Ok(squiffy) = squiffy_q.get(item_e.0) else {
            continue;
        };
        let amount = squiffy.amount();
        log_message_er.send(LogMessageEvent(format!(
            "Self-inflicted {} health!",
            amount
        )));
        player_hp.decrease(amount);
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Heave {
    pub base: i32,
    pub modifiers: AbilityModifiers,
    pub target: AbilityTarget,
}

impl Ability for Heave {
    fn name(&self) -> &'static str {
        "Heave"
    }

    fn base(&self) -> i32 {
        self.base
    }

    fn modifiers(&self) -> &AbilityModifiers {
        &self.modifiers
    }

    fn modifiers_mut(&mut self) -> &mut AbilityModifiers {
        &mut self.modifiers
    }
}

fn handle_heave_use(
    mut log_message_er: EventWriter<LogMessageEvent>,
    mut use_item_ev: EventReader<UseItem>,
    mut damage_q: Query<&mut Damage, With<Pointy>>,
    heave_q: Query<&Heave>,
    scroll_q: Query<&Children, With<InventoryScrollUI>>,
) {
    let scroll_children = scroll_q.single();
    for item_e in use_item_ev.read() {
        let Ok(heave) = heave_q.get(item_e.0) else {
            continue;
        };
        let i = scroll_children
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == item_e.0)
            .map(|(i, _)| i)
            .next();
        let Some(scroll_pos) = i else {
            continue;
        };
        let amount = heave.amount();
        let targets = heave
            .target
            .get_targets(scroll_pos, item_e.0, scroll_children.iter());
        for &damage_e in targets.iter() {
            if let Ok(mut damage) = damage_q.get_mut(damage_e) {
                damage.modifiers.entry(item_e.0).or_default().amount += 1;
            }
        }
        log_message_er.send(LogMessageEvent(format!("Heaved {}!", amount)));
    }
}
