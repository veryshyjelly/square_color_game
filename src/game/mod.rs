use bevy::prelude::*;

pub mod boxes;
pub mod logic;
pub mod models;
pub mod prompt;
pub mod score;

use boxes::*;
use models::*;

use crate::AppState;

use self::score::ScorePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Square>()
            .init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(Startup, (spawn_boxes, spawn_game_title).chain())
            .add_plugins(ScorePlugin)
            .add_systems(OnEnter(AppState::Game), spawn_game_menu)
            .add_systems(OnExit(AppState::Game), despawn_game_menu)
            .add_systems(
                Update,
                (change_color_dark, update_game_menu, change_color_light)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component)]
pub struct GameMenu {}

pub fn spawn_game_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    square: ResMut<Square>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            GameMenu {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    square.val.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(150.0),
                    top: Val::Px(150.0),
                    ..default()
                }),
                Prompt {},
            ));
        });
}

pub fn update_game_menu(
    mut prompt_query: Query<&mut Text, With<Prompt>>,
    mut score_up_event: EventReader<ScoreUpEvent>,
    square: Res<Square>,
) {
    for _ in score_up_event.iter() {
        if let Ok(mut prompt) = prompt_query.get_single_mut() {
            prompt.sections[0].value = square.val.clone();
        }
    }
}

pub fn despawn_game_menu(mut commands: Commands, game_menu_query: Query<Entity, With<GameMenu>>) {
    let game_menu = commands
        .get_entity(game_menu_query.get_single().unwrap())
        .unwrap();

    game_menu.despawn_recursive();
}

pub fn spawn_game_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Guess Color",
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: 60.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(50.0),
            top: Val::Px(40.0),
            ..default()
        }),
        GameTitle {},
    ));
}
