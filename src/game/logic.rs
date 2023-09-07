use bevy::prelude::*;

use super::models::{GameOverEvent, Score, ScoreUpEvent, Square, SquareColor};

pub fn check_answer(
    keyboard_input: Res<Input<KeyCode>>,
    score: Res<Score>,
    mut square: ResMut<Square>,
    mut score_up_event: EventWriter<ScoreUpEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Right) && square.color == SquareColor::LIGHT
        || keyboard_input.just_pressed(KeyCode::Left) && square.color == SquareColor::DARK
    {
        let new_square = Square::default();
        (square.val, square.color) = (new_square.val, new_square.color);
        score_up_event.send(ScoreUpEvent { val: score.val + 1 });
    } else if keyboard_input.just_pressed(KeyCode::Right)
        || keyboard_input.just_pressed(KeyCode::Left)
    {
        let new_square = Square::default();
        (square.val, square.color) = (new_square.val, new_square.color);
        game_over_event.send(GameOverEvent {})
    }
}
