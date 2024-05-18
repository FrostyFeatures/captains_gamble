use std::time::Duration;

use bevy::prelude::*;

use crate::{assets::GameFonts, battle::BattleEvent};

const NUMOID_DECAY_TIME: f32 = 1.;
const NUMOID_FONT_SIZE: f32 = 10.;

const NUMOID_POS_COLOR: Color = Color::SEA_GREEN;
const NUMOID_NEG_COLOR: Color = Color::RED;

const NUMOID_PLAYER_POS: Vec2 = Vec2::new(-85., -40.);
const NUMOID_ENEMY_POS: Vec2 = Vec2::new(85., -40.);

pub struct NumoidPlugin;

impl Plugin for NumoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_numoids.run_if(on_event::<BattleEvent>()))
            .add_systems(Update, update_numoids);
    }
}

#[derive(Component)]
struct Numoid {
    color: Color,
    timer: Timer,
}

#[derive(Bundle)]
struct NumoidBundle {
    numoid: Numoid,
    text_bundle: Text2dBundle,
}

impl Numoid {
    pub fn spawn(number: i32, position: Vec2, game_fonts: &GameFonts, commands: &mut Commands) {
        let color = if number > 0 {
            NUMOID_POS_COLOR
        } else {
            NUMOID_NEG_COLOR
        };
        commands.spawn(NumoidBundle {
            numoid: Numoid {
                color,
                timer: Timer::new(Duration::from_secs_f32(NUMOID_DECAY_TIME), TimerMode::Once),
            },
            text_bundle: Text2dBundle {
                text: Text::from_section(
                    if number == 0 {
                        format!("-{}", number)
                    } else if number > 0 {
                        format!("+{}", number)
                    } else {
                        format!("{}", number)
                    },
                    TextStyle {
                        font: game_fonts.font.clone(),
                        font_size: NUMOID_FONT_SIZE,
                        color,
                    },
                ),
                transform: Transform::from_translation(Vec3::from((position, 20.))),
                ..default()
            },
        });
    }
}

fn spawn_numoids(
    mut commands: Commands,
    mut battle_event_er: EventReader<BattleEvent>,
    game_fonts: Res<GameFonts>,
) {
    for event in battle_event_er.read() {
        match event {
            BattleEvent::PlayerHurt(amount) => {
                Numoid::spawn(-*amount, NUMOID_PLAYER_POS, &game_fonts, &mut commands)
            }
            BattleEvent::EnemyHurt(amount) => {
                Numoid::spawn(-*amount, NUMOID_ENEMY_POS, &game_fonts, &mut commands)
            }
            BattleEvent::PlayerHeal(amount) => {
                Numoid::spawn(*amount, NUMOID_PLAYER_POS, &game_fonts, &mut commands)
            }
            _ => (),
        }
    }
}

fn update_numoids(
    mut commands: Commands,
    mut numoids_q: Query<(Entity, &mut Numoid, &mut Transform, &mut Text)>,
    time: Res<Time>,
) {
    for (entity, mut numoid, mut transform, mut text) in numoids_q.iter_mut() {
        numoid.timer.tick(time.delta());
        let timer_percent = numoid.timer.elapsed_secs() / numoid.timer.duration().as_secs_f32();

        transform.translation += Vec3::new(0., time.delta_seconds() * 5., 0.);
        text.sections[0].style.color = numoid.color.with_a(1. - timer_percent);

        if numoid.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
