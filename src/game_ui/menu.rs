use bevy::prelude::*;

use super::{
    NodeLeftSide,
    NodeCenterSide,
    NodeRightSide,
};

// Constants
const MENU_TEXT_SIZE: f32 = 48.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<NodeCenterSide>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let text_height = window.height() / 5.;

    let text = commands.spawn((TextBundle::from_section(
            "Press Space or M1!",
            TextStyle {
                font: asset_server.load("fonts/slkscrb.ttf"),
                font_size: MENU_TEXT_SIZE,
                color: crate::TEXT_COLOR,
        },
    ).with_style(Style {
        margin: UiRect {
            top: Val::Px(text_height),
            ..default()
        },
        ..default()
    }),

    MenuText,
    )).id();

    let window_ui_node = query.single();
    commands
        .entity(window_ui_node)
        .add_child(text);
}

#[derive(Component)]
pub struct MenuText;

pub fn remove_menu_text(
    mut commands: Commands,
    text_query: Query<Entity, With<MenuText>>,
    node_query: Query<Entity, With<NodeCenterSide>>
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
