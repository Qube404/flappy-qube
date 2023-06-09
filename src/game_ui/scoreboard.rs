use bevy::prelude::*;

use super::{
    WindowUiNode,
    Inner,
};

// Constants
const SCOREBOARD_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, (With<WindowUiNode>, With<Inner>)>
) {
    let text = commands.spawn((TextBundle::from_section(
            "Score",
            TextStyle {
                font: asset_server.load("fonts/slkscrb.ttf"),
                font_size: SCOREBOARD_TEXT_SIZE,
                color: crate::TEXT_COLOR,
        },
    ),
    ScoreboardText,
    )).id();

    let window_ui_node = query.single();
    commands
        .entity(window_ui_node)
        .add_child(text);
}

// Components, Resources, Events
#[derive(Resource)]
pub struct Scoreboard {
    pub score: i128,
}

#[derive(Component)]
pub struct ScoreboardText;

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();

    text.sections[0].value = scoreboard.score.to_string();
}

pub fn remove_scoreboard_text(
    text_query: Query<Entity, With<ScoreboardText>>,
    node_query: Query<Entity, (With<WindowUiNode>, With<Inner>)>,
    mut commands: Commands,
) {
    let text = text_query.single();
    let node = node_query.single();

    commands
        .entity(node)
        .remove_children(&[text]);

    commands
        .entity(text)
        .despawn();
}
