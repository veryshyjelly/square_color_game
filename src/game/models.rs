use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct SquareLabel {}

#[derive(Component)]
pub struct GameTitle {}

#[derive(Component)]
pub struct DarkSqure {}

#[derive(Component)]
pub struct LightSquare {}

#[derive(Component)]
pub struct Prompt {}

#[derive(Resource)]
pub struct Square {
    pub val: String,
    pub color: SquareColor,
}

#[derive(PartialEq)]
pub enum SquareColor {
    DARK,
    LIGHT,
}

impl Default for Square {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..8);
        let col = rng.gen_range(0..8);
        if (row % 2 == 0 && col % 2 == 0) || (row % 2 == 1 && col % 2 == 1) {
            Square {
                val: format!(
                    "{}{}",
                    "ABCDEFGH".chars().nth(row).unwrap(),
                    (1..=8).nth(col).unwrap().to_string()
                ),
                color: SquareColor::DARK,
            }
        } else {
            Square {
                val: format!(
                    "{}{}",
                    "ABCDEFGH".chars().nth(row).unwrap(),
                    (1..=8).nth(col).unwrap().to_string()
                ),
                color: SquareColor::LIGHT,
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub val: i32,
}

#[derive(Resource, Default)]
pub struct HighScore {
    pub val: i32,
}

#[derive(Event)]
pub struct ScoreUpEvent {
    pub val: i32,
}

#[derive(Event)]
pub struct GameOverEvent {}

#[derive(Component)]
pub struct ScoreBoard {}

#[derive(Component)]
pub struct HighScoreBoard {}
