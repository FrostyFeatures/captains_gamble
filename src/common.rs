use core::fmt;

use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

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
