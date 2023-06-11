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

fn log_pipes(
    pipe_query: Query<(&Transform, &Velocity, &Offset, &NumberOf), With<Pipe>>,
    mut time_query: Query<&mut DebugTimer>,
) {
    let mut timer = time_query.single_mut(); 
    for (transform, velocity, offset, num) in &pipe_query {
        println!("Transform: {:?}", transform);
        println!("Velocity: {:?}", velocity);
        println!("Offset: {:?}", offset);
        println!("Num: {:?}", num);
    }
}
