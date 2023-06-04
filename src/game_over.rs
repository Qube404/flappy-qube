use bevy::prelude::*;

use super::{
    Collider,
    scoreboard::Scoreboard,
    bird::Bird,
    pipes::Offset,
    pipes::PointMarker,
    pipes::BeenAdded,
    pipes::StartingPosition,
    pipes::Pipe,
    pipes::PIPE_X_SIZE,
};

use rand::prelude::*;

// Restarts the game when a collision event is recieved
pub fn game_over(
    mut bird_query: Query<&mut Transform, (With<Bird>, Without<Collider>)>,
    mut pipes_query: Query<(&mut Transform, &Offset, &mut StartingPosition, Option<&PointMarker>, Option<&mut BeenAdded>), (With<Collider>, Without<Bird>)>,
    mut score: ResMut<Scoreboard>,
    collision_event: EventReader<super::bird::BirdCollisionEvent>,
) {
    let mut rand = thread_rng();
    let mut pipe_height = rand
        .gen_range(300..=800) as f32;

    if !collision_event.is_empty() {
        score.score = 0;

        let mut bird_transform = bird_query.single_mut();
        bird_transform.translation.y = 0.;

        for (mut pipe_transform, offset, starting_position, point_marker, been_added) in &mut pipes_query {
            pipe_transform.translation.y = pipe_height + offset.0;

            if point_marker.is_some() {
                pipe_transform.translation.x = starting_position.0.x + PIPE_X_SIZE / 2.;
                been_added
                    .expect("Should be Some<T>")
                    .0 = false;

                // Resetting random height after resetting a point marker because the order
                // bevy resets items is pipe, pipe, point and since I want the random
                // number to change every pipe set (pipe set is a top pipe, bottom pipe and
                // a point marker) then resetting the random number at the end of a point
                // marker reset should give each pipe set their own unique random height.
                pipe_height = rand
                    .gen_range(300..=800) as f32;
            } else {
                pipe_transform.translation.x = starting_position.0.x;
            }
        } 
    }
}
