use bevy::prelude::*;

pub mod fps;
pub mod menu;
pub mod scoreboard;

pub fn setup(
    mut commands: Commands
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
    WindowUiNode,
    Outer,
    ))

    .with_children(|parent| {
        parent.spawn((NodeBundle {
            style: Style {
                padding: UiRect {
                    top: Val::Percent(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        WindowUiNode,
        Inner,
        ));
    });
}

#[derive(Component)]
pub struct WindowUiNode;

#[derive(Component)]
struct Outer;

#[derive(Component)]
pub struct Inner;
