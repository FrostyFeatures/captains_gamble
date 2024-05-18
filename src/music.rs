use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{assets::GameAudio, AppState};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitGame), spawn_music);
    }
}

fn spawn_music(mut commands: Commands, game_audio: Res<GameAudio>) {
    commands.spawn(AudioBundle {
        source: game_audio.music.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.1),
            ..default()
        },
    });
}
