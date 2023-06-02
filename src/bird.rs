use bevy::prelude::*;
use bevy::sprite::{
    MaterialMesh2dBundle,
    collide_aabb::collide,
};

use super::{
    Velocity, 
    SpeedCap, 
    GravityCap,
    TIME_STEP,
    GRAVITY,
    Collider,
    CollisionEvent,
    scoreboard::Scoreboard,
    pipes::Pipe,
};

// Constants
const BIRD_SIZE: f32 = 30.;
const BIRD_JUMP: f32 = 800.;
const BIRD_COLOR: Color = Color::rgb(0.8, 0.8, 0.2);

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
                translation: Vec3::new(0., 0., 2.),
                scale: Vec3::new(BIRD_SIZE, BIRD_SIZE, 1.),
                ..default()
            },
            ..default()
        },

        super::Velocity(
            Vec2::new(0., 0.)
        ),

        super::GravityCap(-70.),
        super::SpeedCap(
            Vec2::new(0., 1500. * TIME_STEP)
        ),

        Bird,
    ));
}

#[derive(Component)]
pub struct Bird;

// Player Movement: Add to birds velocity when space is pressed
pub fn move_bird(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Velocity, &SpeedCap), With<Bird>>,
) {
    let (mut bird_velocity, speed_cap) = query.single_mut();
    
    if 
        keyboard_input.just_pressed(KeyCode::Space) ||
        mouse_input.just_pressed(MouseButton::Left)
    {
        // Caps the velocity.
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

pub fn bird_pipe_collisions(
    mut bird_query: Query<&Transform, With<Bird>>, 
    collider_query: Query<&Transform, (With<Collider>, With<Pipe>)>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let bird_transform = bird_query.single_mut();

    // Collision check
    for pipe_transform in &collider_query {
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

        // Checks if bird is past pipe and adds to score if it is
        if bird_transform.translation.x > pipe_transform.translation.x {
            scoreboard.score += 1;
        } 
    }
}

// Apply the velocity's calculated in other systems to the transforms
// of the game entities
pub fn apply_bird_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Bird>>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
