use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
) {
    // Camera
    commands
        .spawn(Camera2dBundle::default());
}
