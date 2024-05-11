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

    pub fn is_alive(&self) -> bool {
        !self.is_dead()
    }

    pub fn increase(&mut self, amount: i32) {
        self.current = self.max.min(self.current + amount);
    }

    pub fn decrease(&mut self, amount: i32) {
        self.current -= amount;
    }
}
