use core::fmt;

use bevy::prelude::*;

use crate::tooltip::{TooltipComponent, TooltipSection, TooltipSectionIndex};

#[derive(Component, Clone, Debug)]
pub struct Name(pub String);

impl TooltipComponent for Name {
    fn get_tooltip_section(&self) -> TooltipSection {
        TooltipSection::default_color(self.0.clone(), TooltipSectionIndex::Header)
    }
}

#[derive(Component)]
pub struct Hp {
    pub max: i32,
    pub current: i32,
}

impl Hp {
    pub fn new(max: i32) -> Self {
        Self { max, current: max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn increase(&mut self, amount: i32) {
        self.current = self.max.min(self.current + amount);
    }

    pub fn decrease(&mut self, amount: i32) {
        self.current -= amount;
    }

    pub fn health_bar_index(&self) -> usize {
        usize::min(((self.current as f32 / self.max as f32) * 59.) as usize, 59)
    }
}

impl fmt::Display for Hp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.current, self.max)
    }
}
