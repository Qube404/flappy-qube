#![allow(dead_code)]
/// A version of flappy bird

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    sprite::collide_aabb::{collide, Collision},
};

/// Constants: Setting up constant game values
const TIME_STEP: f32 = 1. / 60.;
const GRAVITY: f32 = -40.;

// Player properties
const BIRD_SIZE: f32 = 30.;
const BIRD_JUMP: f32 = 700. * 1.4;
const BIRD_POSITION: Vec3 = Vec3::new(0., 0., 1.);

// Pipe Properties
const PIPE_X_SIZE: f32 = 100.;
const PIPE_Y_SIZE: f32 = 2000.;

// Scoreboard properties
const SCOREBOARD_TOP_PADDING: Val = Val::Px(500.);
const SCOREBOARD_LEFT_PADDING: Val = Val::Px(500.);

// Colors
const BIRD_COLOR: Color = Color::rgb(0.8, 0.8, 0.2);
const PIPE_COLOR: Color = Color::rgb(0.1, 0.7, 0.2);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.5, 0.9);

/// Main: Adding systems, resources, schedules, etc to game App
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_systems(
            (
                apply_bird_gravity,

                move_bird.after(apply_bird_gravity),

                apply_velocity
                    .after(apply_bird_gravity)
                    .after(move_bird),

                check_for_collisions
                    .after(apply_bird_gravity)
                    .after(move_bird)
                    .after(apply_velocity),

                update_scoreboard
                    .after(check_for_collisions),

                game_over
                    .after(check_for_collisions),

                move_pipes
                    .before(apply_velocity)
                    .before(check_for_collisions),
            )
            .in_schedule(CoreSchedule::FixedUpdate),
        )
        .run();
}

/// Setup: Adding entities to the game world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands
        .spawn(Camera2dBundle::default());

    // Player Bird
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BIRD_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(BIRD_COLOR)),
                transform: Transform::from_translation(BIRD_POSITION),
                ..default()
            },

            Velocity(
                Vec2::new(0., 0.)
            ),

            GravityCap(-70.),
            SpeedCap(
                Vec2::new(0., 1500. * TIME_STEP)
            ),

            Bird,
        ));

    // Pipes
    commands
        .spawn((
            PipeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Box::new(PIPE_X_SIZE, PIPE_Y_SIZE, 0.).into()).into(),
                    material: materials.add(ColorMaterial::from(PIPE_COLOR)),
                    transform: Transform::from_translation(Vec3::new(500., 0., 0.)),
                    ..default()
                },

                velocity: Velocity(
                    Vec2::new(0., 0.)
                ),
                collider: Collider, 
                pipe: Pipe,
            },
        ));

    // Score
    commands
        .spawn((
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 56.0,
                    color: TEXT_COLOR,
                }
            )
            .with_style(
                Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: SCOREBOARD_TOP_PADDING,
                        left: SCOREBOARD_LEFT_PADDING,
                        ..default()
                    },
                    ..default()
                }
            ),
            ScoreboardText,
        ));

}

#[derive(Component)]
struct Bird;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Pipe;

#[derive(Bundle)]
struct PipeBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    pipe: Pipe,
}

#[derive(Component, Deref, DerefMut)]
struct GravityCap(f32);

#[derive(Component, Deref, DerefMut)]
struct SpeedCap(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Resource)]
struct Scoreboard {
    score: i128,
}

#[derive(Component)]
struct ScoreboardText;

// Player Movement: Add to birds velocity when space is pressed
fn move_bird(
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

// Pipe Movement: Add a constant value to pipes velocity.
// It's better to isolate this to a system rather then hardcode
// the value in the entity so that a more complex movement
// system can be added later.
fn move_pipes(
    mut query: Query<&mut Velocity, With<Pipe>>,
) {
    for mut pipe_velocity in &mut query {
        pipe_velocity.x = -50. * TIME_STEP;
    }
}

// Apply gravity to player's velocity
fn apply_bird_gravity(
    mut query: Query<(&mut Velocity, &GravityCap), With<Bird>>
) {
    let (mut bird_velocity, gravity_cap) = query.single_mut(); 

    // Caps the velocity.
    if bird_velocity.y > **gravity_cap {
        bird_velocity.y += GRAVITY * TIME_STEP;
    }
}

// Apply the velocity's calculated in other systems to the transforms
// of the game entities
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = scoreboard.score.to_string();
}

fn check_for_collisions(
    mut bird_query: Query<&Transform, With<Bird>>, 
    collider_query: Query<&Transform, (With<Collider>, With<Pipe>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let bird_transform = bird_query.single_mut();
    let bird_size = bird_transform.scale.truncate();

    // Collision check
    for pipe_transform in &collider_query {
        let collision_x = bird_transform.translation.x - pipe_transform.translation.x;

        println!("Collision: {:?}\nBird: {:?}\nPipe: {:?}", collision_x, bird_transform, pipe_transform);
        if collision_x < 1. && collision_x > -1. {
            collision_events.send_default();
        }
    }
}

fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_event: EventReader<CollisionEvent>,
) {
    if !collision_event.is_empty() {
        commands
            .spawn(
                TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 56.0,
                        color: TEXT_COLOR,
                    }
                )
                .with_style(
                    Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(1.),
                            left: Val::Px(1.),
                            right: Val::Px(1.),
                            bottom: Val::Px(1.),
                        },
                        ..default()
                    }
                )
            );
    }
}
