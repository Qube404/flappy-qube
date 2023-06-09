use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};

use super::{
    Inner,
    WindowUiNode,
};

// Constants
const FPS_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, (With<WindowUiNode>, With<Inner>)>,
    mut fps_spawned: ResMut<FpsSpawned>,
) {
    if fps_spawned.0 == false {
        let text = commands.spawn((TextBundle::from_section(
                "FPS",
                TextStyle {
                    font: asset_server.load("fonts/slkscrb.ttf"),
                    font_size: FPS_TEXT_SIZE,
                    color: crate::TEXT_COLOR,
            },
        ),
        FpsText,
        )).id();

        let window_ui_node = query.single();
        commands
            .entity(window_ui_node)
            .add_child(text);

        fps_spawned.0 = true;
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Resource)]
pub struct FpsSpawned(pub bool);

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
