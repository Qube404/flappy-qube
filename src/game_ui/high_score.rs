use std::{fs::File, io::{Write, Read}};

use bevy::{
    prelude::*,
    window::WindowCloseRequested,
};

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
pub struct HighScoreLoaded(pub bool);

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
pub fn load_high_score(
    mut highscore: ResMut<HighScore>,
    mut loaded: ResMut<HighScoreLoaded>,
) {
    if loaded.0 == true {
        return;
    }

    if let Ok(mut file) = File::open("highscore.txt") {
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Should be able to read contents");

        highscore.highscore = contents.parse::<i128>()
            .expect("Should be a valid i128 value");
    }

    loaded.0 = true;
}

// Save high score on close
pub fn save_high_score(
    highscore: Res<HighScore>,
    score: Res<Scoreboard>,
) {
    if score.score > highscore.highscore {
        return;
    }

    // Truncates file if it exists.
    let mut file = File::create("highscore.txt")
        .expect("Should be able to create file");

    let save_text = highscore
        .highscore
        .to_string();

    file
        .write_all(save_text.as_bytes())
        .expect("Should be able to write to file");
}
