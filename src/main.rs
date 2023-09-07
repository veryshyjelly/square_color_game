mod game;
mod main_menu;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowResolution},
};
use game::GamePlugin;
use main_menu::MainMenuPlugin;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 400.0;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Square Color".into(),
            resizable: false,
            transparent: true,
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Update, toggle_state)
        .add_systems(Update, exit_game)
        .run()
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event.send(AppExit);
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

pub fn toggle_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if app_state.eq(&AppState::Game) {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
        }
        if app_state.eq(&AppState::MainMenu) {
            commands.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}
