use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Press space\nto play.",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(105.0),
                    top: Val::Px(160.0),
                    ..default()
                }),
            );
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    let main_menu = commands
        .get_entity(main_menu_query.get_single().unwrap())
        .unwrap();
    main_menu.despawn_recursive();
}
