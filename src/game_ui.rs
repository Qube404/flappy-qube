use bevy::prelude::*;

pub mod fps;
pub mod menu;
pub mod scoreboard;
pub mod high_score;

pub fn setup(
    mut commands: Commands
) {
    commands.spawn((NodeBundle {
        style: Style {
            flex_basis: Val::Percent(100.),
            
            // Horizontal, Left-Right
            justify_content: JustifyContent::SpaceBetween,

            // Vertical, Up-Down
            align_items: AlignItems::Start,

            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        ..default()
    },
    WindowUiNode,
    ))

    .with_children(|parent| {

        // Left Side UI Node
        parent.spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    max_size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            NodeLeftSide,
        ));

        // Center UI Node
        parent.spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),

                    // A max size doesnt limit the size of the text but allows
                    // it to overflow without also increasing the size of the node
                    // and thus pushing other nodes around.
                    max_size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            NodeCenterSide,
        ));

        // Right Side UI Node
        parent.spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    max_size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            NodeRightSide,
        ));
    });
}

#[derive(Component)]
pub struct WindowUiNode;

#[derive(Component)]
pub struct NodeLeftSide;

#[derive(Component)]
pub struct NodeCenterSide;

#[derive(Component)]
pub struct NodeRightSide;
