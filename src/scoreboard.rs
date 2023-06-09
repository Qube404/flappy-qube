use bevy::prelude::*;

// Constants
const SCOREBOARD_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((NodeBundle {
        style: Style {
            flex_basis: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    },
    ScoreboardNode,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                padding: UiRect {
                    top: Val::Percent(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/slkscrb.ttf"),
                    font_size: SCOREBOARD_TEXT_SIZE,
                    color: super::TEXT_COLOR,
                }
            ),
            ScoreboardText,
            ));
        });
    });
}

// Components, Resources, Events
#[derive(Resource)]
pub struct Scoreboard {
    pub score: i128,
}

#[derive(Component)]
pub struct ScoreboardText;

#[derive(Component)]
pub struct ScoreboardNode;

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();

    text.sections[0].value = scoreboard.score.to_string();
}

pub fn remove_scoreboard_text(
    node_query: Query<Entity, With<ScoreboardNode>>,
    mut commands: Commands,
) {
    let node = node_query.single();

    commands
        .entity(node)
        .despawn_recursive();
}
