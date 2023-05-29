#![allow(dead_code)]
/// A version of flappy bird

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

/// Constants: Setting up constant game values
const TIME_STEP: f32 = 1. / 60.;
const GRAVITY: f32 = -40.;

// Player properties
const BIRD_SIZE: f32 = 30.;
const BIRD_JUMP: f32 = 700.;
const BIRD_POSITION: Vec3 = Vec3::new(0., 0., 0.);

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
        .add_systems(
            (
                apply_bird_gravity,
                move_bird.after(apply_bird_gravity),
                apply_velocity
                    .after(apply_bird_gravity)
                    .after(move_bird)
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

    // Score
    commands
        .spawn(
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
                )
        );

}

#[derive(Component)]
struct Bird;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Pipe;

#[derive(Component, Deref, DerefMut)]
struct GravityCap(f32);

#[derive(Component, Deref, DerefMut)]
struct SpeedCap(Vec2);

#[derive(Resource)]
struct Scoreboard {
    score: i128,
}

// Player Movement
fn move_bird(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &SpeedCap), With<Bird>>,
) {
    let (mut bird_velocity, speed_cap) = query.single_mut();
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        if bird_velocity.y < speed_cap.y {
            //bird_velocity.y += pressed * BIRD_JUMP * TIME_STEP;
            bird_velocity.y = 1.3 * BIRD_JUMP * TIME_STEP;
        }
    }

}

fn apply_bird_gravity(
    mut query: Query<(&mut Velocity, &GravityCap), With<Bird>>
) {
    let (mut bird_velocity, gravity_cap) = query.single_mut(); 

    if bird_velocity.y > **gravity_cap {
        bird_velocity.y += GRAVITY * TIME_STEP;
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
