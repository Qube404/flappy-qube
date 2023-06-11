use bevy::prelude::*;

use crate::{
    Velocity,
    pipes::*,
};

const TIME_STEP: f32 = 1.;

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(DebugTimer(Timer::from_seconds(1., TimerMode::Repeating)));
}

#[derive(Component)]
pub struct DebugTimer(Timer);

pub fn log_points(
    pipe_query: Query<(&Velocity, &Offset, &NumberOf, &BeenAdded), (With<PointMarker>, Changed<BeenAdded>)>,
) {
    for (velocity, offset, num, been_added) in &pipe_query {
        if been_added.0 == true {
            println!("----------------");
            println!("Point Num: {:?}", num.0);
            println!("  Velocity: {:?}", velocity.0);
            println!("  Offset: {:?}", offset.0);
            println!("  Been Added: {:?}", been_added.0);
            println!("----------------");
            println!("\n");
        }
    }
}
