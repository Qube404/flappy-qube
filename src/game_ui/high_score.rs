use std::fs::File;

use bevy::{prelude::*, app::AppExit};

use super::{
    NodeLeftSide,
    NodeCenterSide,
    NodeRightSide,

    scoreboard::Scoreboard,
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

            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/slkscrb.ttf"),
                    font_size: FPS_TEXT_SIZE,
                    color: crate::TEXT_COLOR,
                },
            ),
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

#[derive(Resource)]
pub struct HighScore {
    pub highscore: i128,
}

pub fn update_highscore(
    score: Res<Scoreboard>,
    mut highscore: ResMut<HighScore>,
) {
    if score.score > highscore.highscore {
        highscore.highscore = score.score;  
    }  
}

pub fn update_highscore_text(
    highscore: Res<HighScore>,
    mut highscore_text: Query<&mut Text, With<HighScoreText>>,
) {
    let mut text = highscore_text.single_mut();
    text.sections[1].value = highscore.highscore.to_string();
}

// Load high score on open

// Save high score on close
pub fn save_high_score(
    exit_event: EventReader<AppExit>
) {
}
