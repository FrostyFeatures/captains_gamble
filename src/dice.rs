use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Die, NormalDie>();
    }
}

#[bevy_trait_query::queryable]
pub trait Die {
    fn roll(&self, rng: &mut ThreadRng, queue_count: usize) -> usize;
}

#[derive(Component)]
pub struct NormalDie;

impl NormalDie {
    pub fn spawn_new(commands: &mut Commands) -> Entity {
        commands.spawn(Self).id()
    }
}

impl Die for NormalDie {
    fn roll(&self, rng: &mut ThreadRng, queue_count: usize) -> usize {
        rng.gen_range(0..queue_count)
    }
}
