use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use super::{
    Velocity, 
    TIME_STEP,
    GRAVITY,
    Collider,
    AppState,
    scoreboard::Scoreboard,
    pipes::Pipe,
    pipes::PointMarker,
    pipes::BeenAdded,
    pipes::PIPE_X_SIZE,
    pipes::PIPE_Y_SIZE,
};

// Constants
const BIRD_SIZE: f32 = 80.;
const BIRD_SCALE: Vec3 = Vec3::new(BIRD_SIZE, BIRD_SIZE, 1.);
const BIRD_JUMP: f32 = 800.;
const BIRD_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 2.);
const BIRD_COLOR: Color = Color::rgb(0.8, 0.8, 0.2);

const GRAVITY_CAP: f32 = -70.;
const SPEED_CAP: Vec2 = Vec2::new(0., 1500. * TIME_STEP);

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bird_handle = asset_server.load("sprites/FlappyQube.png");

    commands.spawn((
        SpriteBundle {
            texture: bird_handle,
            transform: Transform {
                translation: BIRD_STARTING_POSITION,
                scale: Vec3::new(4., 4., 0.),
                ..default()
            },
            ..default()
        },

        super::Velocity(Vec2::new(0., 0.)),

        GravityCap(GRAVITY_CAP),
        SpeedCap(SPEED_CAP),

        Bird,
    ));
}

// Components, Resources, Events
#[derive(Component)]
pub struct Bird;

#[derive(Component, Deref, DerefMut)]
pub struct GravityCap(f32);

#[derive(Component, Deref, DerefMut)]
pub struct SpeedCap(Vec2);

#[derive(Default)]
pub struct BirdCollisionEvent;

// Player movement by adding to birds velocity
pub fn move_bird(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Velocity, &SpeedCap), With<Bird>>,
) {
    let (mut bird_velocity, speed_cap) = query.single_mut();
    
    // Uses just_pressed instead of pressed so the fly button 
    // can't be held down
    if 
        keyboard_input.just_pressed(KeyCode::Space) ||
        mouse_input.just_pressed(MouseButton::Left)
    {
        // Caps the velocity so spamming doesn't
        // endlessly speed up the player
        if bird_velocity.y < speed_cap.y {
            bird_velocity.y = BIRD_JUMP * TIME_STEP;
        }
    }
}

// Apply gravity to player's velocity
pub fn apply_bird_gravity(
    mut query: Query<(&mut Velocity, &GravityCap), With<Bird>>
) {
    let (mut bird_velocity, gravity_cap) = query.single_mut(); 

    // Caps the velocity.
    if bird_velocity.y > **gravity_cap {
        bird_velocity.y += GRAVITY * TIME_STEP;
    }
}

pub fn rotate_bird(
    mut query: Query<(&mut Transform, &Velocity), With<Bird>>,
) {
    let (mut transform, velocity) = query.single_mut();

    let mut percentage: f32 = velocity.y / SPEED_CAP.y;

    percentage = percentage.max(-1.0);
    percentage = percentage.min(1.0);

    percentage = (percentage + 1.0) * 0.5;

    let max_rotation = Quat::from_rotation_z(f32::to_radians(80.));
    let min_rotation = Quat::from_rotation_z(f32::to_radians(-80.));

    transform.rotation = min_rotation.lerp(max_rotation, percentage);
}

// Check for collisions with pipes
pub fn bird_pipe_collisions(
    mut bird_query: Query<&Transform, With<Bird>>, 
    collider_query: Query<&Transform, (With<Collider>, With<Pipe>)>,
    mut collision_events: EventWriter<BirdCollisionEvent>,
) {
    let bird_transform = bird_query.single_mut();

    // Collision check
    //
    // Checks the bird with a scale of 1 so that the game
    // is more forgiving.
    for pipe_transform in &collider_query {
        // Collision checking function
        let collision = collide(
            bird_transform.translation,
            Vec2::new(20., 20.),
            pipe_transform.translation,
            Vec2::new(PIPE_X_SIZE, PIPE_Y_SIZE),
        );

        // If there was a collision send a collision event
        if collision.is_some() {
            collision_events.send_default();
        }
    }
}

// Check for collisions with point markers
pub fn bird_point_collisions(
    mut bird_query: Query<&Transform, With<Bird>>, 
    mut point_query: Query<(&Transform, &mut BeenAdded), (With<Collider>, With<PointMarker>)>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let bird_transform = bird_query.single_mut();

    for (point_transform, mut been_added) in &mut point_query {
        let collision = collide(
            bird_transform.translation,
            bird_transform.scale.truncate(),
            point_transform.translation,
            point_transform.scale.truncate(),
        );

        if collision.is_some() && **been_added == false {
            scoreboard.score += 1;
            **been_added = true;
        }
    }
}

// Apply velocity to birds transform
pub fn apply_bird_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Bird>>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

// Starts game
pub fn game_start(
    mut query: Query<(&mut Velocity, &SpeedCap), With<Bird>>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let (mut bird_velocity, speed_cap) = query.single_mut();
    
    // This does the same thing as the normal movement system
    // with the added feature of setting the game state to 
    // AppState::InGame.
    if 
        keyboard_input.just_pressed(KeyCode::Space) ||
        mouse_input.just_pressed(MouseButton::Left)
    {
        if bird_velocity.y < speed_cap.y {
            bird_velocity.y = BIRD_JUMP * TIME_STEP;
            next_state.set(AppState::InGame);
        }
    }
}
