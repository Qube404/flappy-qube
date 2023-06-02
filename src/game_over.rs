use bevy::prelude::*;

use super::{
    Collider,
    scoreboard::Scoreboard,
    bird::Bird,
    pipes::Offset,
    pipes::PointMarker,
    pipes::BeenAdded,
    pipes::PIPE_X_SIZE,
};

use rand::prelude::*;

// Restarts the game when a collision event is recieved
pub fn game_over(
    mut bird_query: Query<&mut Transform, With<Bird>>,
    mut pipes_query: Query<(&mut Transform, &Offset, Option<&PointMarker>, Option<&mut BeenAdded>), With<Collider>>,
    mut score: ResMut<Scoreboard>,
    collision_event: EventReader<super::bird::BirdCollisionEvent>,
) {
    if !collision_event.is_empty() {
        score.score = 0;

        let mut bird_transform = bird_query.single_mut();
        bird_transform.translation.y = 0.;

        let pipe_height = thread_rng()
            .gen_range(300..=800) as f32;

        let mut i = 0;
        for (mut pipe_transform, offset, point_marker, been_added) in &mut pipes_query {
            let x_pos = pipe_transform.translation.x;

            if x_pos <= -1000.  {
                pipe_transform.translation.x = i as f32 * 500.;
                pipe_transform.translation.y = pipe_height + offset.0;
            } else if x_pos <= -1000. + PIPE_X_SIZE / 2. && point_marker.is_some() {
                pipe_transform.translation.x = i as f32 * 500. + PIPE_X_SIZE / 2.;
                pipe_transform.translation.y = pipe_height + offset.0;
                been_added
                    .expect("Should be Some<T>")
                    .0 = false;
            }

            i += 1;
        } 
    }
}
