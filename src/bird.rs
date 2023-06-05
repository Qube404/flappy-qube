use bevy::prelude::*;
use bevy::sprite::{
    MaterialMesh2dBundle,
    collide_aabb::collide,
};

use super::{
    Velocity, 
    TIME_STEP,
    GRAVITY,
    Collider,
    scoreboard::Scoreboard,
    pipes::Pipe,
    pipes::PointMarker,
    pipes::BeenAdded,
};

// Constants
const BIRD_SIZE: f32 = 30.;
const BIRD_SCALE: Vec3 = Vec3::new(BIRD_SIZE, BIRD_SIZE, 1.);
const BIRD_JUMP: f32 = 800.;
const BIRD_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 2.);
const BIRD_COLOR: Color = Color::rgb(0.8, 0.8, 0.2);

const GRAVITY_CAP: f32 = -70.;
const SPEED_CAP: Vec2 = Vec2::new(0., 1500. * TIME_STEP);

// Initial Setup
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.).into()).into(),
            material: materials.add(ColorMaterial::from(BIRD_COLOR)),
            transform: Transform {
                translation: BIRD_STARTING_POSITION,
                scale: BIRD_SCALE,
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

// Check for collisions with pipes
pub fn bird_pipe_collisions(
    mut bird_query: Query<&Transform, With<Bird>>, 
    collider_query: Query<&Transform, (With<Collider>, With<Pipe>)>,
    mut collision_events: EventWriter<BirdCollisionEvent>,
) {
    let bird_transform = bird_query.single_mut();

    // Collision check
    for pipe_transform in &collider_query {
        // Collision checking function
        let collision = collide(
            bird_transform.translation,
            bird_transform.scale.truncate(),
            pipe_transform.translation,
            pipe_transform.scale.truncate(),
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
