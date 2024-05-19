use bevy::prelude::*;

use crate::tooltip::{TooltipComponent, TooltipSection, TooltipSectionIndex};

use super::abilities::{AbilityModifier, AbilityTarget, Damage, TargetFilter};

pub(super) struct AttributePlugin;

pub const POINTY: &str = "Pointy";
pub const FLINTLOCK: &str = "Flintlock";
pub const PELLETS: &str = "Pellets";
pub const CANNONBALL: &str = "Pellets";

impl Plugin for AttributePlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Attribute, Pointy>();
        app.register_component_as::<dyn Attribute, Flintlock>();
        app.register_component_as::<dyn Attribute, Pellets>();
        app.register_component_as::<dyn Attribute, Cannonball>();
    }
}

#[bevy_trait_query::queryable]
pub trait Attribute: TooltipComponent {
    fn name(&self) -> &'static str;
    fn _get_tooltip_section(&self) -> TooltipSection {
        let text = format!("{}", self.name());
        TooltipSection::default_color(text, TooltipSectionIndex::Footer)
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Pointy;

impl Attribute for Pointy {
    fn name(&self) -> &'static str {
        POINTY
    }
}

impl TooltipComponent for Pointy {
    fn get_tooltip_section(&self) -> TooltipSection {
        self._get_tooltip_section()
    }
}

#[derive(Component, Clone, Debug)]
pub struct Flintlock {
    ammo: String,
    capacity: usize,
    loaded: usize,
}

impl Flintlock {
    pub fn empty(ammo: String, capacity: usize) -> Self {
        Self {
            ammo,
            capacity,
            loaded: 0,
        }
    }

    pub fn can_load(&self, ammo: &String) -> bool {
        ammo.contains(&self.ammo)
    }

    pub fn full(&self) -> bool {
        self.loaded >= self.capacity
    }

    pub fn load(&mut self, amount: usize) {
        self.loaded = (self.loaded + amount).min(self.capacity);
    }

    pub fn fire(&mut self) -> bool {
        if self.loaded == 0 {
            return false;
        } else {
        }
        self.loaded -= 1;
        true
    }
}

impl Attribute for Flintlock {
    fn name(&self) -> &'static str {
        FLINTLOCK
    }
}

impl TooltipComponent for Flintlock {
    fn get_tooltip_section(&self) -> TooltipSection {
        let text = format!(
            "{} ({}/{} {})",
            self.name(),
            self.loaded,
            self.capacity,
            self.ammo
        );
        TooltipSection::default_color(text, TooltipSectionIndex::Footer)
    }
}

#[derive(Component, Clone, Debug)]
pub struct Pellets {
    pub load_amount: usize,
    pub target: AbilityTarget,
}

impl Attribute for Pellets {
    fn name(&self) -> &'static str {
        PELLETS
    }
}

impl TooltipComponent for Pellets {
    fn get_tooltip_section(&self) -> TooltipSection {
        let text = format!(
            "{} (Loads {} [{}] {})",
            self.name(),
            self.load_amount,
            self.target.filter.name(),
            self.target.attribute,
        );
        TooltipSection::default_color(text, TooltipSectionIndex::Footer)
    }
}

#[derive(Component, Clone, Debug)]
pub struct Cannonball {
    pub load_amount: usize,
    pub target: AbilityTarget,
}

impl Attribute for Cannonball {
    fn name(&self) -> &'static str {
        CANNONBALL
    }
}

impl TooltipComponent for Cannonball {
    fn get_tooltip_section(&self) -> TooltipSection {
        let text = format!(
            "{} (Loads {} [{}] {})",
            self.name(),
            self.load_amount,
            self.target.filter.name(),
            self.target.attribute,
        );
        TooltipSection::default_color(text, TooltipSectionIndex::Footer)
    }
}
