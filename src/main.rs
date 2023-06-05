/// A version of flappy bird

use bevy::prelude::*;

mod bird;
mod pipes;
mod scoreboard;
mod camera;
mod game_over;

/// Constants
const TIME_STEP: f32 = 1. / 60.;
const GRAVITY: f32 = -40.;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.5, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .insert_resource(scoreboard::Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(bird::setup)
        .add_startup_system(pipes::setup)
        .add_startup_system(scoreboard::setup)
        .add_startup_system(camera::setup)
        .add_event::<bird::BirdCollisionEvent>()
        .add_systems(
            (
                bird::apply_bird_gravity,

                bird::move_bird,
                pipes::move_pipes,

                bird::apply_bird_velocity,
                pipes::apply_pipes_velocity,

                bird::bird_pipe_collisions,
                bird::bird_point_collisions,

                scoreboard::update_scoreboard,

                game_over::game_over,
            )
            .in_schedule(CoreSchedule::FixedUpdate),
        )
        .run();
}

// Components & Resources used by more then one module outside main
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Debug, Clone, Copy, Eq, Default, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}
