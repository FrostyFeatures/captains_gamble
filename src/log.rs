use bevy::prelude::*;

use crate::{assets::GameFonts, AppState};

const FONT_SIZE: f32 = 6.;
const FONT_COLOR: Color = Color::WHITE;

const MAX_MESSAGES: usize = 5;

pub struct BattleLogPlugin;

impl Plugin for BattleLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogMessageEvent>()
            .add_systems(OnEnter(AppState::Battling), setup_log)
            .add_systems(OnExit(AppState::Battling), destroy_log)
            .add_systems(
                Update,
                (handle_log_message_events, cleanup_logs)
                    .chain()
                    .run_if(in_state(AppState::Battling)),
            );
    }
}

#[derive(Event)]
pub struct LogMessageEvent(pub String);

#[derive(Component)]
struct LogMessage;

#[derive(Component)]
struct LogRoot;

fn handle_log_message_events(
    mut commands: Commands,
    mut log_message_event_er: EventReader<LogMessageEvent>,
    game_fonts: Res<GameFonts>,
    log_root_q: Query<Entity, With<LogRoot>>,
) {
    let Ok(log_root_e) = log_root_q.get_single() else {
        return;
    };

    commands.entity(log_root_e).with_children(|parent| {
        for event in log_message_event_er.read() {
            parent.spawn((
                LogMessage,
                TextBundle {
                    text: Text::from_section(
                        event.0.clone(),
                        TextStyle {
                            color: FONT_COLOR,
                            font_size: FONT_SIZE,
                            font: game_fonts.font.clone(),
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        }
    });
}

fn cleanup_logs(mut commands: Commands, log_messages_q: Query<&Children, With<LogRoot>>) {
    let Ok(log_children) = log_messages_q.get_single() else {
        return;
    };

    log_children
        .iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| *i >= MAX_MESSAGES)
        .for_each(|(_, e)| {
            commands.get_entity(*e).map(|e| e.despawn_recursive());
        })
}

fn setup_log(mut commands: Commands) {
    commands.spawn((
        LogRoot,
        NodeBundle {
            z_index: ZIndex::Global(i32::MAX),
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                overflow: Overflow::clip_x(),
                width: Val::Px(100.),
                height: Val::Percent(100.),
                top: Val::Percent(50.),
                padding: UiRect::all(Val::Px(4.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
    ));
}

fn destroy_log(mut commands: Commands, log_root_q: Query<Entity, With<LogRoot>>) {
    for log_root_e in log_root_q.iter() {
        commands
            .get_entity(log_root_e)
            .map(|e| e.despawn_recursive());
    }
}
