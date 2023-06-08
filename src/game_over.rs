use bevy::prelude::*;

use super::{
    AppState,
    Collider,
    scoreboard::Scoreboard,
    bird::Bird,
    pipes::Offset,
    pipes::PointMarker,
    pipes::BeenAdded,
    pipes::StartingPosition,
    pipes::NumberOf,
    pipes::PIPE_X_SIZE,
    pipes::PIPE_AMOUNT,
    pipes::PIPE_HEIGHT_RANGE,
};

use rand::prelude::*;

// Restarts the game when a collision event is recieved
pub fn game_over(
    mut bird_query: Query<&mut Transform, (With<Bird>, Without<Collider>)>,
    mut pipes_query: Query<(
        &mut Transform, 
        &Offset, 
        &mut StartingPosition, 
        &NumberOf,
        Option<&PointMarker>, 
        Option<&mut BeenAdded
    >), 
        (With<Collider>, Without<Bird>)>,
    mut score: ResMut<Scoreboard>,
    collision_event: EventReader<super::bird::BirdCollisionEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut rand = thread_rng();
    let mut random_heights: Vec<f32> = Vec::new();

    for _ in 1..=PIPE_AMOUNT {
        random_heights
            .push(rand.gen_range(PIPE_HEIGHT_RANGE) as f32);
    }

    if !collision_event.is_empty() {
        // Score
        score.score = 0;

        // Bird
        let mut bird_transform = bird_query.single_mut();
        bird_transform.translation.y = 0.;
        bird_transform.rotation.z = 0.;

        // Pipes
        for (
            mut pipe_transform, 
            offset, 
            starting_position, 
            number_of,
            point_marker, 
            been_added
        ) in &mut pipes_query {
            pipe_transform.translation.y = random_heights
                    .get(number_of.0 - 1)
                    .expect("Should be a valid index in random_heights") + offset.0;

            if point_marker.is_some() {
                pipe_transform.translation.x = starting_position.0.x + PIPE_X_SIZE / 2.;
                been_added
                    .expect("Should be Some<T>")
                    .0 = false;
            } else {
                pipe_transform.translation.x = starting_position.0.x;
            }
        } 
    }

    next_state.set(AppState::MainMenu);
}
