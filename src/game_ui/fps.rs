use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};

use super::{
    NodeLeftSide,
    NodeCenterSide,
    NodeRightSide,
};

// Constants
const FPS_TEXT_SIZE: f32 = 48.;
pub const TIME_STEP: f32 = 1.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fps_spawned: ResMut<FpsSpawned>,
    node_query: Query<Entity, With<NodeLeftSide>>,
) {
    if fps_spawned.0 == true {
        return;
    }

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

        FpsTime(Timer::from_seconds(1., TimerMode::Repeating)),
        FpsText,
    )).id();

    let node = node_query.single();

    commands
        .entity(node)
        .add_child(text);

    fps_spawned.0 = true;
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct FpsNode;

#[derive(Resource)]
pub struct FpsSpawned(pub bool);

#[derive(Component)]
pub struct FpsTime(Timer);

pub fn update_fps(
    mut fps_query: Query<(&mut Text, &mut FpsTime), With<FpsText>>,
    diagnostics: Res<Diagnostics>,
    time: Res<Time>,
) {
    let (mut text, mut timer) = fps_query.single_mut();
    
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
    {
        if let Some(value) = fps.smoothed() {
            text.sections[1].value = (value as i32).to_string();
        }
    }
}
