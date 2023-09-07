use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppState;

use super::models::{GameOverEvent, HighScore, HighScoreBoard, Score, ScoreBoard, ScoreUpEvent};

use super::logic::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreUpEvent>()
            .add_event::<GameOverEvent>()
            .add_systems(Startup, spawn_score_board)
            .add_systems(
                Update,
                (check_answer, update_score_board, update_high_score_board)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

pub fn spawn_score_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        TextBundle::from_section(
            format!("highscore: {}", high_score.val.to_string()),
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: 20.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(window.height() - 20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        HighScoreBoard {},
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("score: {}", score.val.to_string()),
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: 20.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(window.height() - 20.0),
            right: Val::Px(40.0),
            ..default()
        }),
        ScoreBoard {},
    ));
}

fn update_score_board(
    mut score_board_query: Query<&mut Text, With<ScoreBoard>>,
    mut score: ResMut<Score>,
    mut score_up_event: EventReader<ScoreUpEvent>,
) {
    for score_up in score_up_event.iter() {
        if let Ok(mut score_board) = score_board_query.get_single_mut() {
            score.val = score_up.val;
            score_board.sections[0].value = format!("score: {}", score.val.to_string());
        }
    }
}

fn update_high_score_board(
    mut commands: Commands,
    mut score_board_query: Query<&mut Text, With<ScoreBoard>>,
    mut high_score_board_query: Query<&mut Text, With<HighScoreBoard>>,
    mut high_score: ResMut<HighScore>,
    mut score: ResMut<Score>,
    mut game_over_event: EventReader<GameOverEvent>,
) {
    for _ in game_over_event.iter() {
        if let Ok(mut score_board) = high_score_board_query.get_single_mut() {
            high_score.val = high_score.val.max(score.val);
            score_board.sections[0].value = format!("high score: {}", high_score.val.to_string());
        }
        if let Ok(mut score_board) = score_board_query.get_single_mut() {
            score.val = 0;
            score_board.sections[0].value = format!("score: {}", score.val.to_string());
        }
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}
