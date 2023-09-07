use super::models::*;
use bevy::prelude::*;

const BLOCK_SIZE: f32 = 400.0 / 4.0;
const BOX_COLOR_DARK: Color = Color::rgb(174.0 / 256.0, 154.0 / 256.0, 137.0 / 256.0);
const BOX_COLOR_DARKER: Color = Color::rgb(122.0 / 256.0, 108.0 / 256.0, 96.0 / 256.0);
const BOX_COLOR_LIGHT: Color = Color::rgb(235.0 / 256.0, 209.0 / 256.0, 184.0 / 256.0);
const BOX_COLOR_LIGHER: Color = Color::rgb(255.0 / 256.0, 230.0 / 256.0, 199.0 / 256.0);

pub fn spawn_boxes(mut commands: Commands) {
    let mut light_blocks = vec![];
    let mut dark_blocks = vec![];

    for i in 0..4 {
        for j in 0..4 {
            let mut block = NodeBundle {
                style: Style {
                    width: Val::Px(BLOCK_SIZE),
                    height: Val::Px(BLOCK_SIZE),
                    position_type: PositionType::Absolute,
                    left: Val::Px(BLOCK_SIZE * (j as f32)),
                    top: Val::Px(BLOCK_SIZE * (i as f32)),
                    ..default()
                },
                ..default()
            };
            if (i % 2 == 0 && j % 2 == 0) || (i % 2 == 1 && j % 2 == 1) {
                block.background_color = BackgroundColor(BOX_COLOR_DARK);
                light_blocks.push(block);
            } else {
                block.background_color = BackgroundColor(BOX_COLOR_LIGHT);
                dark_blocks.push(block);
            }
        }
    }

    light_blocks.into_iter().for_each(|block| {
        commands.spawn((block, LightSquare {}));
    });

    dark_blocks.into_iter().for_each(|block| {
        commands.spawn((block, DarkSqure {}));
    });
}

pub fn change_color_dark(
    keyboard_input: Res<Input<KeyCode>>,
    mut block_query: Query<&mut BackgroundColor, With<DarkSqure>>,
) {
    if keyboard_input.pressed(KeyCode::Left) {
        for mut block in block_query.iter_mut() {
            block.0 = BOX_COLOR_DARKER;
        }
    } else {
        for mut block in block_query.iter_mut() {
            block.0 = BOX_COLOR_DARK;
        }
    }
}

pub fn change_color_light(
    keyboard_input: Res<Input<KeyCode>>,
    mut block_query: Query<&mut BackgroundColor, With<LightSquare>>,
) {
    if keyboard_input.pressed(KeyCode::Right) {
        for mut block in block_query.iter_mut() {
            block.0 = BOX_COLOR_LIGHER;
        }
    } else {
        for mut block in block_query.iter_mut() {
            block.0 = BOX_COLOR_LIGHT;
        }
    }
}
