/// A version of flappy bird

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    sprite::Mesh2dHandle,
    sprite::collide_aabb::collide,
};

use rand::prelude::*;

/// Constants: Setting up constant game values
const TIME_STEP: f32 = 1. / 60.;
const GRAVITY: f32 = -40.;

// Player properties
const BIRD_SIZE: f32 = 30.;
const BIRD_JUMP: f32 = 800.;

// Pipe Properties
const PIPE_X_SIZE: f32 = 100.;
const PIPE_Y_SIZE: f32 = 800.;
const PIPE_DIFF: f32 = 1100.;
const PIPE_AMOUNT: i32 = 6;

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

                /*
                spawn_lines
                    .after(move_pipes),

                despawn_lines
                    .after(spawn_lines)
                    .after(move_pipes),
                */
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
                mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                material: materials.add(ColorMaterial::from(BIRD_COLOR)),
                transform: Transform {
                    translation: Vec3::new(0., 0., 2.),
                    scale: Vec3::new(BIRD_SIZE, BIRD_SIZE, 1.),
                    ..default()
                },
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
    for i in 1..=PIPE_AMOUNT {
        let random_height: i32 = thread_rng().gen_range(300..=800); 
        let pipe_height = random_height as f32;

        let color = i as f32 * 0.1;
        let color = Color::rgb(color, color, color);
        commands
            .spawn((
                PipeBundle {
                    mesh_bundle: MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                        material: materials.add(ColorMaterial::from(color/*PIPE_COLOR*/)),
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
                TopPipe,
            ));

        commands
            .spawn((
                PipeBundle {
                    mesh_bundle: MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                        material: materials.add(ColorMaterial::from(color/*PIPE_COLOR*/)),
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
                BottomPipe,
            ));

        commands
            .spawn((
                PipePointBundle {
                    mesh_bundle: MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                        material: materials.add(ColorMaterial::from(color/*Color::rgba(0., 0., 0., 1.)*/)),
                        transform: Transform {
                            translation: Vec3::new(i as f32 * 500. + PIPE_X_SIZE / 2., pipe_height - PIPE_DIFF / 2., 1.),
                            scale: Vec3::new(10., 1100. - PIPE_Y_SIZE, 0.),
                            ..default()
                        },
                        ..default()
                    },

                    velocity: Velocity(
                        Vec2::new(0., 0.)
                    ),

                    collider: Collider,
                    offset: Offset(-PIPE_DIFF / 2.),
                },
                PointPipe,
            ));
    }

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
    offset: Offset,
    pipe: Pipe,
}

#[derive(Bundle)]
struct PipePointBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    offset: Offset,
}

#[derive(Component)]
struct TopPipe;

#[derive(Component)]
struct BottomPipe;

#[derive(Component)]
struct PointPipe;

#[derive(Component, Deref, DerefMut)]
struct GravityCap(f32);

#[derive(Component, Deref, DerefMut)]
struct SpeedCap(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Offset(f32);

#[derive(Resource)]
struct Scoreboard {
    score: i128,
}

#[derive(Component)]
struct ScoreboardText;

#[derive(Component)]
struct MarkerLine;

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
//
// Why the fuck does a copied value of a randomly generated
// number change throughout different iterations of a loop?!?!
fn move_pipes(
    mut query_top_pipes: 
        Query<
            (&mut Velocity, &mut Transform, &Offset), 
            (With<TopPipe>, Without<BottomPipe>, Without<PointPipe>)
        >, 
    mut query_bottom_pipes: 
        Query<
            (&mut Velocity, &mut Transform, &Offset), 
            (With<BottomPipe>, Without<TopPipe>, Without<PointPipe>)
        >, 
    mut query_point_pipes: 
        Query<
            (&mut Velocity, &mut Transform, &Offset), 
            (With<PointPipe>, Without<TopPipe>, Without<BottomPipe>)
        >, 
) {
    let mut rand: ThreadRng = thread_rng(); 
    let mut pipe_heights: Vec<f32> = Vec::new();

    for i in 0..PIPE_AMOUNT {
        pipe_heights.push(rand.gen_range(300..=800) as f32);
    }

    let mut i = 0;
    for (mut velocity, mut transform, offset) in query_point_pipes.iter_mut() {
        velocity.x = -500./*150.*/ * TIME_STEP;
        let pipe_height = pipe_heights.get(i).unwrap() + offset.0;

        if transform.translation.x <= -1000. {
            transform.translation.x = 2000.;
            transform.translation.y = pipe_height;
        }
        i += 1;
        assert_eq!(pipe_height, pipe_heights.get(i).unwrap() + offset.0);
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

// Debugging Functions
//
// Spawns appropriate lines for showing interaction between game entities
fn spawn_lines(
    collider_query: Query<&Transform, (With<Collider>, With<Pipe>)>,
    bird_query: Query<&Transform, With<Bird>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
) {
    let bird_transform = bird_query.single();
    for pipe_transform in &collider_query {
        let line_x_position = pipe_transform.translation.x - PIPE_X_SIZE / 2.;


        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(5., 1100., 2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0., 0., 0.))),
            transform: Transform::from_translation(Vec3::new(line_x_position, 0., 2.)),
            ..default()
        });

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(5., 5., 2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1., 0., 0.))),
            transform: Transform::from_translation(pipe_transform.translation),
            ..default()
        });

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(5., 5., 2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1., 0., 0.))),
            transform: Transform::from_translation(bird_transform.translation),
            ..default()
        });

        if !collision_events.is_empty() {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Box::new(5., 1100., 2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(1., 0., 0.))),
                transform: Transform::from_translation(Vec3::new(line_x_position, 0., 2.)),
                ..default()
                },

                MarkerLine,
            ));
        }
    }

}

// Despawns the lines spawned by spawn_lines
fn despawn_lines(
    query: Query<Entity, (With<Mesh2dHandle>, Without<Pipe>, Without<Bird>, Without<MarkerLine>)>,
    mut commands: Commands,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
