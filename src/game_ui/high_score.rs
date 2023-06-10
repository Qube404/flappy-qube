use bevy::prelude::*;

use super::{
    NodeLeftSide,
    NodeCenterSide,
    NodeRightSide,
};

// Constants
const FPS_TEXT_SIZE: f32 = 48.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut high_score_spawned: ResMut<HighScoreSpawned>,
    window_node_query: Query<Entity, With<NodeRightSide>>,
) {
    if high_score_spawned.0 == true {
        return;
    }

    let text = commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "High: ",
                TextStyle {
                    font: asset_server.load("fonts/slkscrb.ttf"),
                    font_size: FPS_TEXT_SIZE,
                    color: crate::TEXT_COLOR,
                },
            ),

            TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/slkscrb.ttf"),
                    font_size: FPS_TEXT_SIZE,
                    color: crate::TEXT_COLOR,
            }),
        ]),

        HighScoreText,
    )).id();

    let window_ui_node = window_node_query.single();

    commands
        .entity(window_ui_node)
        .add_child(text);

    high_score_spawned.0 = true;
}

#[derive(Component)]
pub struct HighScoreText;

#[derive(Resource)]
pub struct HighScoreSpawned(pub bool);
