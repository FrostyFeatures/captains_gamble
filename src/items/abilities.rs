use bevy::{prelude::*, utils::HashMap};

use crate::tooltip::{TooltipComponent, TooltipSection};

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn TooltipComponent, Damage>();
        app.register_component_as::<dyn TooltipComponent, Jolly>();
        app.register_component_as::<dyn TooltipComponent, Squiffy>();
        app.register_component_as::<dyn TooltipComponent, Heave>();
        app.register_component_as::<dyn TooltipComponent, SeaLegs>();

        app.register_component_as::<dyn Ability, Damage>();
        app.register_component_as::<dyn Ability, Jolly>();
        app.register_component_as::<dyn Ability, Squiffy>();
        app.register_component_as::<dyn Ability, Heave>();
        app.register_component_as::<dyn Ability, SeaLegs>();

        // app.add_systems(OnExit(AppState::Battling), clear_ability_modifiers);
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AbilityModifier {
    pub amount: i32,
}

pub type AbilityModifiers = HashMap<Entity, AbilityModifier>;

#[bevy_trait_query::queryable]
pub trait Ability {
    fn name(&self) -> String;
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
    T: Ability,
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

#[derive(Default, Clone, Debug)]
pub struct AbilityTarget {
    pub filter: TargetFilter,
    pub attribute: String,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum TargetFilter {
    #[default]
    All,
    Next(usize),
    Prev(usize),
    Neighbours,
    AllNext,
    AllPrev,
}

impl TargetFilter {
    fn name(&self) -> String {
        match self {
            TargetFilter::All => "ALL".to_string(),
            TargetFilter::Next(n) => format!("NEXT {n}"),
            TargetFilter::Prev(n) => format!("PREV {n}"),
            TargetFilter::Neighbours => "NEIGHBOURS".to_string(),
            TargetFilter::AllNext => "ALL NEXT".to_string(),
            TargetFilter::AllPrev => "ALL PREV".to_string(),
        }
    }

    pub fn get_targets<'a>(
        &self,
        index: usize,
        entity: Entity,
        list: impl IntoIterator<Item = &'a Entity>,
    ) -> Vec<Entity> {
        let iter = list.into_iter().enumerate().filter(|(_, c)| **c != entity);
        let targets: Vec<(usize, &Entity)> = match self {
            TargetFilter::All => iter.collect(),
            TargetFilter::AllNext => iter.filter(|(i, _)| *i > index).collect(),
            TargetFilter::Next(n) => iter
                .filter(|(i, _)| *i > index && *i <= index + n)
                .collect(),
            TargetFilter::AllPrev => iter.filter(|(i, _)| *i < index).collect(),
            TargetFilter::Prev(n) => iter
                .filter(|(i, _)| *i < index && *i + n >= index)
                .collect(),
            TargetFilter::Neighbours => iter
                .filter(|(i, _)| *i + 1 == index || *i == index + 1)
                .collect(),
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

impl Damage {
    pub fn new(base: i32) -> Self {
        Self { base, ..default() }
    }
}

impl Ability for Damage {
    fn name(&self) -> String {
        "Damage".to_string()
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

#[derive(Component, Default, Debug, Clone)]
pub struct Jolly {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl Jolly {
    pub fn new(base: i32) -> Self {
        Self { base, ..default() }
    }
}

impl Ability for Jolly {
    fn name(&self) -> String {
        "Jolly".to_string()
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

#[derive(Component, Default, Debug, Clone)]
pub struct Squiffy {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl Squiffy {
    pub fn new(base: i32) -> Self {
        Self { base, ..default() }
    }
}

impl Ability for Squiffy {
    fn name(&self) -> String {
        "Squiffy".to_string()
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

#[derive(Component, Default, Clone, Debug)]
pub struct Heave {
    pub base: i32,
    pub modifiers: AbilityModifiers,
    pub target: AbilityTarget,
}

impl Heave {
    pub fn new(base: i32, target: AbilityTarget) -> Self {
        Self { base, ..default() }
    }
}

impl Ability for Heave {
    fn name(&self) -> String {
        format!(
            "Heave ({} {})",
            self.target.filter.name(),
            self.target.attribute
        )
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

#[derive(Component, Default, Clone, Debug)]
pub struct SeaLegs {
    pub base: i32,
    pub modifiers: AbilityModifiers,
}

impl SeaLegs {
    pub fn new(base: i32) -> Self {
        Self { base, ..default() }
    }
}

impl Ability for SeaLegs {
    fn name(&self) -> String {
        "Sea legs".to_string()
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
