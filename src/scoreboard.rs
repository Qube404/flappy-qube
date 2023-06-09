use bevy::prelude::*;

// Constants
const SCOREBOARD_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Text
    let scoreboard_text = commands.spawn(TextBundle::from_section(
        "0",
        TextStyle {
            font: asset_server.load("fonts/slkscrb.ttf"),
            font_size: SCOREBOARD_TEXT_SIZE,
            color: super::TEXT_COLOR,
        }
    )).id();

    // Inner Node
    let inner_scoreboard_node = commands.spawn((NodeBundle {
        style: Style {
            padding: UiRect {
                top: Val::Percent(10.),
                ..default()
            },
            ..default()
        },
        ..default()
    },
    ScoreboardText(scoreboard_text),
    )).id();

    // Outer Node
    commands.spawn((NodeBundle {
        style: Style {
            flex_basis: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    },
    ScoreboardNode(inner_scoreboard_node),
    ));

    // Old
    /*commands.spawn((NodeBundle {
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
    });*/
}

// Components, Resources, Events
#[derive(Resource)]
pub struct Scoreboard {
    pub score: i128,
}

#[derive(Component)]
pub struct ScoreboardText(Entity);

#[derive(Component)]
pub struct ScoreboardNode(Entity);

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text>,
    scoreboard_query: Query<Entity, With<ScoreboardText>>,
) {
    let text_entity = scoreboard_query.single();
    let text = &mut query
        .get_mut(text_entity)
        .expect("Should be a valid text entity.");

    text.sections[0].value = scoreboard.score.to_string();
}

pub fn remove_scoreboard_text(
    node_query: Query<Entity, With<ScoreboardNode>>,
    text_query: Query<Entity, With<ScoreboardText>>, 
    mut commands: Commands,
) {
    let node = node_query.single();
    let text = text_query.single();

    commands
        .entity(node)
        .despawn();

    commands
        .entity(text)
        .despawn();
}
