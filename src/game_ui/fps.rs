use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};

use super::WindowUiNode;

// Constants
const FPS_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_node_query: Query<Entity, With<WindowUiNode>>,
    mut fps_spawned: ResMut<FpsSpawned>,
) {
    if fps_spawned.0 == false {
        let text = commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/slkscrb.ttf"),
                        font_size: FPS_TEXT_SIZE,
                        color: crate::TEXT_COLOR,
                    },
                ),

                TextSection::from_style(TextStyle {
                        font: asset_server.load("fonts/slkscrb.ttf"),
                        font_size: FPS_TEXT_SIZE,
                        color: crate::TEXT_COLOR,
                }),
            ]),
            FpsText,
        )).id();

        let window_ui_node = window_node_query.single();

        commands
            .entity(window_ui_node)
            .add_child(text);

        fps_spawned.0 = true;
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct FpsNode;

#[derive(Resource)]
pub struct FpsSpawned(pub bool);

pub fn update_fps(
    mut query: Query<&mut Text, With<FpsText>>,
    diagnostics: Res<Diagnostics>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[1].value = format!("{value:.2}");
            println!("{value:.2}");
        }
    }
}
