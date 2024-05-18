use std::time::Duration;

use bevy::prelude::*;

use crate::{assets::GameSprites, battle::BattleEvent, AppState, GAME_HEIGHT};

const FLOOR_HEIGHT: f32 = 21.;
const PIRATE_HEIGHT: f32 = 32.;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalAnimationTimer {
            timer: Timer::new(Duration::from_secs_f32(0.8), TimerMode::Repeating),
        })
        .add_systems(OnEnter(AppState::InitGame), setup_scene)
        .add_systems(OnEnter(AppState::GameStart), spawn_player_pirate)
        .add_systems(OnEnter(AppState::Battling), spawn_enemy_pirate)
        .add_systems(OnEnter(AppState::GameStart), despawn_enemy_pirate)
        .add_systems(OnEnter(AppState::OrganizeInventory), despawn_enemy_pirate)
        .add_systems(OnEnter(AppState::GameOver), despawn_player_pirate)
        .add_systems(
            Update,
            handle_player_damanged.run_if(any_with_component::<PlayerPirate>),
        )
        .add_systems(
            Update,
            (handle_enemy_damanged, handle_enemy_attack).run_if(any_with_component::<EnemyPirate>),
        )
        .add_systems(
            Update,
            (
                update_animation_timers,
                update_animation_flashes,
                update_animation_overrides,
            ),
        );
    }
}

#[derive(Component)]
struct PlayerPirate;

#[derive(Component)]
struct EnemyPirate;

#[derive(Resource)]
struct GlobalAnimationTimer {
    timer: Timer,
}

#[derive(Component)]
struct AnimationTimer {
    frames: usize,
}

#[derive(Component)]
struct AnimationOverride(Timer);

#[derive(Component)]
struct AnimationFlash {
    color: Color,
    timer: Timer,
}

fn setup_scene(mut commands: Commands, game_sprites: Res<GameSprites>) {
    // background
    commands.spawn(SpriteBundle {
        texture: game_sprites.background.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        ..default()
    });

    // floor
    commands.spawn(SpriteBundle {
        texture: game_sprites.floor.clone(),
        transform: Transform::from_translation(Vec3::new(
            0.,
            -GAME_HEIGHT / 2. + FLOOR_HEIGHT / 2.,
            0.,
        )),
        ..default()
    });
}

fn spawn_player_pirate(mut commands: Commands, game_sprites: Res<GameSprites>) {
    commands.spawn((
        PlayerPirate,
        AnimationTimer { frames: 2 },
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(
                -100.,
                -GAME_HEIGHT / 2. + FLOOR_HEIGHT + PIRATE_HEIGHT / 2.,
                0.,
            )),
            texture: game_sprites.pirate_sheet.clone(),
            atlas: TextureAtlas {
                layout: game_sprites.pirate_layout.clone(),
                index: 0,
            },
            ..default()
        },
    ));
}

fn despawn_player_pirate(
    mut commands: Commands,
    player_pirate_q: Query<Entity, With<PlayerPirate>>,
) {
    for player_pirate in player_pirate_q.iter() {
        commands.entity(player_pirate).despawn_recursive();
    }
}

fn spawn_enemy_pirate(mut commands: Commands, game_sprites: Res<GameSprites>) {
    commands.spawn((
        EnemyPirate,
        AnimationTimer { frames: 2 },
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(
                100.,
                -GAME_HEIGHT / 2. + FLOOR_HEIGHT + PIRATE_HEIGHT / 2.,
                0.,
            )),
            texture: game_sprites.skeleton_sheet.clone(),
            atlas: TextureAtlas {
                layout: game_sprites.skeleton_layout.clone(),
                index: 0,
            },
            ..default()
        },
    ));
}

fn despawn_enemy_pirate(mut commands: Commands, enemy_pirate_q: Query<Entity, With<EnemyPirate>>) {
    for enemy_pirate in enemy_pirate_q.iter() {
        commands.entity(enemy_pirate).despawn_recursive();
    }
}

fn update_animation_timers(
    mut timer_q: Query<(&mut TextureAtlas, &AnimationTimer), Without<AnimationOverride>>,
    mut global_timer: ResMut<GlobalAnimationTimer>,
    time: Res<Time>,
) {
    global_timer.timer.tick(time.delta());
    if global_timer.timer.just_finished() {
        for (mut texture_atlas, timer) in timer_q.iter_mut() {
            texture_atlas.index =
                (time.elapsed_seconds() / global_timer.timer.duration().as_secs_f32()) as usize
                    % timer.frames;
        }
    }
}

fn update_animation_overrides(
    mut commands: Commands,
    mut overrides_q: Query<(Entity, &mut AnimationOverride)>,
    time: Res<Time>,
) {
    for (entity, mut r#override) in overrides_q.iter_mut() {
        r#override.0.tick(time.delta());

        if r#override.0.finished() {
            commands.entity(entity).remove::<AnimationOverride>();
        }
    }
}

fn handle_player_damanged(
    mut battle_event_er: EventReader<BattleEvent>,
    mut commands: Commands,
    player_sprite_q: Query<Entity, With<PlayerPirate>>,
) {
    let player_sprite = player_sprite_q.single();
    for event in battle_event_er.read() {
        if !matches!(event, BattleEvent::PlayerHurt(_)) {
            continue;
        }
        commands.entity(player_sprite).insert(AnimationFlash {
            color: Color::RED,
            timer: Timer::new(Duration::from_secs_f32(0.3), TimerMode::Once),
        });
    }
}

fn handle_enemy_damanged(
    mut battle_event_er: EventReader<BattleEvent>,
    mut commands: Commands,
    enemy_sprite_q: Query<Entity, With<EnemyPirate>>,
) {
    let enemy_sprite = enemy_sprite_q.single();
    for event in battle_event_er.read() {
        if !matches!(event, BattleEvent::EnemyHurt(_)) {
            continue;
        }
        commands.entity(enemy_sprite).insert(AnimationFlash {
            color: Color::RED,
            timer: Timer::new(Duration::from_secs_f32(0.3), TimerMode::Once),
        });
    }
}

fn handle_enemy_attack(
    mut commands: Commands,
    mut battle_event_er: EventReader<BattleEvent>,
    mut enemy_sprite_q: Query<(Entity, &mut TextureAtlas), With<EnemyPirate>>,
) {
    let (entity, mut atlas) = enemy_sprite_q.single_mut();
    for event in battle_event_er.read() {
        if !matches!(event, BattleEvent::EnemyAttack) {
            continue;
        }

        atlas.index = 2;
        commands.entity(entity).insert(AnimationOverride(Timer::new(
            Duration::from_secs_f32(0.2),
            TimerMode::Once,
        )));
    }
}

fn update_animation_flashes(
    mut commands: Commands,
    mut animation_flash_q: Query<(Entity, &mut AnimationFlash, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut flash, mut sprite) in animation_flash_q.iter_mut() {
        let alpha = flash.timer.elapsed().as_secs_f32() / flash.timer.duration().as_secs_f32();
        sprite.color = flash.color.with_l(alpha * 0.5 + 0.5);

        flash.timer.tick(time.delta());

        if flash.timer.finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<AnimationFlash>();
        }
    }
}
