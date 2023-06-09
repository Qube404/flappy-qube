use bevy::{
    prelude::*,

    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};

// Constants
const FPS_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((NodeBundle {
        style: Style {
            flex_basis: Val::Percent(100.),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    },
    FpsNode,
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
                    font_size: FPS_TEXT_SIZE,
                    color: super::TEXT_COLOR,
                }
            ),
            FpsText,
            ));
        });
    });
}

#[derive(Component)]
pub struct FpsNode;

#[derive(Component)]
pub struct FpsText;

pub fn update_fps(
    mut query: Query<&mut Text, With<FpsText>>,
    diagnostics: Res<Diagnostics>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("{value:.2}");
        }
    }
}









