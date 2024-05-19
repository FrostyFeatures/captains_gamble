use bevy::prelude::*;
use rand::rngs::ThreadRng;

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_rng);
    }
}

#[derive(Clone)]
pub struct Rng(pub ThreadRng);

fn init_rng(world: &mut World) {
    world.insert_non_send_resource(Rng(rand::thread_rng()));
}
