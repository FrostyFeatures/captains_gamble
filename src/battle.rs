use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(BattleState::default())
            .add_systems(OnEnter(BattleState::Start), setup_battle);
    }
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum BattleState {
    #[default]
    Start,
    PlayerTurn,
    EnemyTurn,
    End,
}

fn setup_battle(
    mut commands: Commands,
) {

}
