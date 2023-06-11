use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use rand::prelude::*;

use std::ops::RangeInclusive;

use super::{
    Velocity, 
    TIME_STEP,
    Collider,
};

// Constants
pub const PIPE_X_SIZE: f32 = 90.;//100
pub const PIPE_Y_SIZE: f32 = 750.;//800

// Always use this for the amount of pipes in game.
pub const PIPE_AMOUNT: i32 = 6;
pub const PIPE_HEIGHT_RANGE: RangeInclusive<i32> = 200..=700;

const PIPE_COLOR: Color = Color::rgb(0.1, 0.7, 0.2);
const POINT_MARKER: Color = Color::rgba(0., 0., 0., 0.);
const PIPE_GAP_X: f32 = 500.;
const PIPE_GAP_Y: f32 = PIPE_Y_SIZE + 250.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let pipe_handle = asset_server.load("sprites/Pipe.png");

    // Spawns three entities per loop iteration. First is the top pipe,
    // second is the bottom pipe and third is the point marker.
    for i in 1..=PIPE_AMOUNT {
        let pipe_height: f32 = thread_rng()
            .gen_range(PIPE_HEIGHT_RANGE) as f32; 

        // Top Pipes
        commands.spawn((
            PipeBundle {
                sprite_bundle: SpriteBundle {
                    texture: pipe_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * PIPE_GAP_X, pipe_height, 1.),
                        scale: Vec3::new(5., 5., 0.),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(Vec2::new(0., 0.)),
                offset: Offset(0.),
                collider: Collider, 
                pipe: Pipe,
                starting_position: StartingPosition(
                    Vec3::new(i as f32 * PIPE_GAP_X, pipe_height, 1.),
                )
            },
            NumberOf(i as usize),
        ));

        // Bottom Pipes
        commands.spawn((
            PipeBundle {
                sprite_bundle: SpriteBundle {
                    texture: pipe_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(i as f32 * PIPE_GAP_X, pipe_height - PIPE_GAP_Y, 1.),
                        scale: Vec3::new(5., 5., 0.),
                        rotation: Quat::from_rotation_x(f32::to_radians(180.)),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(Vec2::new(0., 0.)),
                offset: Offset(-PIPE_GAP_Y),
                collider: Collider, 
                pipe: Pipe,
                starting_position: StartingPosition(
                    Vec3::new(i as f32 * PIPE_GAP_X, pipe_height - PIPE_GAP_Y, 1.),
                )
            },
            NumberOf(i as usize),
        ));

        // Point Markers
        commands.spawn((
            PipePointBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                    material: materials.add(ColorMaterial::from(POINT_MARKER)),
                    transform: Transform {
                        translation: Vec3::new(
                            i as f32 * PIPE_GAP_X + PIPE_X_SIZE / 2.,
                            pipe_height - PIPE_GAP_Y / 2.,
                            1.
                        ),
                        // Increased scale of x to 10 from 1 to account for potential
                        // collision skipping when lagging.
                        scale: Vec3::new(10., PIPE_GAP_Y - PIPE_Y_SIZE, 0.),
                        ..default()
                    },
                    ..default()
                },

                velocity: Velocity(Vec2::new(0., 0.)),
                collider: Collider,
                offset: Offset(-PIPE_GAP_Y / 2.),
                point_marker: PointMarker,
                been_added: BeenAdded(false),
                starting_position: StartingPosition(
                    Vec3::new(i as f32 * PIPE_GAP_X, pipe_height - PIPE_GAP_Y / 2., 1.),
                )
            },
            NumberOf(i as usize),
        ));
    }

}

// Components, Resources, Events
#[derive(Bundle)]
struct PipeBundle {
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    collider: Collider,
    offset: Offset,
    pipe: Pipe,
    starting_position: StartingPosition,
}

#[derive(Bundle)]
struct PipePointBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    offset: Offset,
    point_marker: PointMarker,
    been_added: BeenAdded,
    starting_position: StartingPosition,
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PointMarker;

#[derive(Component, Debug)]
pub struct Offset(pub f32);

#[derive(Component, Debug)]
pub struct BeenAdded(pub bool);

#[derive(Component)]
pub struct StartingPosition(pub Vec3);

#[derive(Component, Debug)]
pub struct NumberOf(pub usize);

// Pipe Movement: Add a constant value to pipes velocity.
// Also moves the pipes to the right edge of the screen as
// they move off the left side.
pub fn move_pipes(
    mut query_pipes: Query<(
        &mut Transform, 
        &mut Velocity, 
        &Offset, 
        Option<&PointMarker>, 
        Option<&mut BeenAdded>,
    ), 
        With<Collider>
    >,
) {
    let pipe_height = thread_rng()
        .gen_range(PIPE_HEIGHT_RANGE) as f32;

    for (mut transform, mut velocity, offset, point_marker, been_added) in &mut query_pipes {
        velocity.x = -300. * TIME_STEP;

        let x_pos = transform.translation.x;

        if x_pos <= -1000.  {
            transform.translation.x = 2000.;
            transform.translation.y = pipe_height + offset.0;
        } else if x_pos <= -1000. + PIPE_X_SIZE / 2. && point_marker.is_some() {
            transform.translation.x = 2000. + PIPE_X_SIZE / 2.;
            transform.translation.y = pipe_height + offset.0;

            let mut been_added = been_added
                .expect("Should be Seme<T>");

            been_added.0 = false;
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
