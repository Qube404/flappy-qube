use bevy::prelude::*;

use super::{
    AppState,
    Collider,
    Velocity,
    game_ui::scoreboard::Scoreboard,
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
    mut bird_query: Query<(&mut Transform, &mut Velocity), (With<Bird>, Without<Collider>)>,
    mut pipes_query: Query<(
        &mut Transform, 
        &mut Velocity,
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

    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut rand = thread_rng();
    let mut random_heights: Vec<f32> = Vec::new();

    for _ in 1..=PIPE_AMOUNT {
        random_heights
            .push(rand.gen_range(PIPE_HEIGHT_RANGE) as f32);
    }

    if !collision_event.is_empty() {
        let game_over_sound = asset_server
            .load("sounds/game_over.mp3");
        audio.play(game_over_sound);

        // Score
        score.score = 0;

        // Bird
        let (mut bird_transform, mut bird_velocity) = bird_query.single_mut();
        bird_transform.translation.y = 0.;
        bird_transform.rotation.z = 0.;
        bird_velocity.0 = Vec2::new(0., 0.);

        // Pipes
        for (
            mut pipe_transform, 
            mut pipe_velocity,
            offset, 
            starting_position, 
            number_of,
            point_marker, 
            been_added
        ) in &mut pipes_query {
            pipe_transform.translation.y = random_heights
                    .get(number_of.0 - 1)
                    .expect("Should be a valid index in random_heights") + offset.0;

            pipe_velocity.0 = Vec2::new(0., 0.,);

            if point_marker.is_some() {
                pipe_transform.translation.x = starting_position.0.x + PIPE_X_SIZE / 2.;
                been_added
                    .expect("Should be Some<T>")
                    .0 = false;
            } else {
                pipe_transform.translation.x = starting_position.0.x;
            }
        } 

        next_state.set(AppState::MainMenu);
    }
}
