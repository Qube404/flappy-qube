use bevy::prelude::*;

// Initial Setup
pub fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

// Moved to a module despite being barebones
// for potential future extensibility.
