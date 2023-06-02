use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use rand::prelude::*;

use super::{
    Velocity, 
    TIME_STEP,
    Collider,
};

// Constants
const PIPE_X_SIZE: f32 = 100.;
const PIPE_Y_SIZE: f32 = 800.;
const PIPE_DIFF: f32 = 1100.;
const PIPE_AMOUNT: i32 = 6;
const PIPE_COLOR: Color = Color::rgb(0.1, 0.7, 0.2);
const POINT_MARKER: Color = Color::rgba(0., 0., 0., 1.);

// Initial Setup
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawns three entities per loop iteration. First is the top pipe,
    // second is the bottom pipe and third is the point marker.
    for i in 1..=PIPE_AMOUNT {
        let pipe_height: f32 = thread_rng().gen_range(300..=800) as f32; 

        // Top Pipes
        commands.spawn((
            PipeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                    material: materials.add(ColorMaterial::from(PIPE_COLOR)),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * 500., pipe_height, 1.),
                        scale: Vec3::new(PIPE_X_SIZE, PIPE_Y_SIZE, 0.),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(
                    Vec2::new(0., 0.)
                ),
                offset: Offset(0.),
                collider: Collider, 
                pipe: Pipe,
            },
        ));

        // Bottom Pipes
        commands.spawn((
            PipeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                    material: materials.add(ColorMaterial::from(PIPE_COLOR)),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * 500., pipe_height - PIPE_DIFF, 1.),
                        scale: Vec3::new(PIPE_X_SIZE, PIPE_Y_SIZE, 0.),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(
                    Vec2::new(0., 0.)
                ),
                offset: Offset(-PIPE_DIFF),
                collider: Collider, 
                pipe: Pipe,
            },
        ));

        // Point Markers
        commands.spawn((
            PipePointBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                    material: materials.add(ColorMaterial::from(POINT_MARKER)),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * 500. + PIPE_X_SIZE / 2., pipe_height - PIPE_DIFF / 2., 1.),
                        scale: Vec3::new(1., 1100. - PIPE_Y_SIZE, 0.),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(
                    Vec2::new(0., 0.)
                ),

                collider: Collider,
                offset: Offset(-PIPE_DIFF / 2.),
                point_marker: PointMarker,
            },
        ));
    }

}

// Components, Resources, Events
#[derive(Bundle)]
struct PipeBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    offset: Offset,
    pipe: Pipe,
}

#[derive(Bundle)]
struct PipePointBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    offset: Offset,
    point_marker: PointMarker,
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PointMarker;

#[derive(Component)]
pub struct Offset(f32);

// Pipe Movement: Add a constant value to pipes velocity.
// It's better to isolate this to a system rather then hardcode
// the value in the entity so that a more complex movement
// system can be added later.
//
// Why the fuck does a copied value of a randomly generated
// number change throughout different iterations of a loop?!?!
pub fn move_pipes(
    mut query_pipes: Query<(&mut Transform, &mut Velocity, &Offset, Option<&PointMarker>), With<Collider>>,
) {
    let pipe_height = thread_rng()
        .gen_range(300..=800) as f32;

    for (mut transform, mut velocity, offset, point_marker) in &mut query_pipes {
        velocity.x = -300. * TIME_STEP;

        let x_pos = transform.translation.x;

        // Notes for tommorow: Problem is with the pipe point hitting
        // later then the pipes because its position is further
        // then the pipes and thus is called at a different time with
        // a different random number.
        if x_pos <= -1000.  {
            transform.translation.x = 2000.;
            transform.translation.y = pipe_height + offset.0;
        } else if x_pos <= -1000. + PIPE_X_SIZE / 2. && point_marker.is_some() {
            transform.translation.x = 2000. + PIPE_X_SIZE / 2.;
            transform.translation.y = pipe_height + offset.0;
        }
    }
}

// Apply the velocity's calculated in other systems to the transforms
// of the game entities
pub fn apply_pipes_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Collider>>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
