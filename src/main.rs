/// A version of flappy bird

use bevy::prelude::*;

mod bird;
mod pipes;
mod scoreboard;
mod camera;
mod game_over;
mod menu;

/// Constants
const TIME_STEP: f32 = 1. / 60.;
const GRAVITY: f32 = -40.;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.5, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // Stops pixel art from being blurry.
            ImagePlugin::default_nearest()
        ))

        .add_state::<AppState>()
        
        .insert_resource(scoreboard::Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))

        .add_event::<bird::BirdCollisionEvent>()

        .add_startup_system(bird::setup)
        .add_startup_system(pipes::setup)
        .add_startup_system(camera::setup)
        .add_startup_system(menu::setup)

        .add_system(scoreboard::setup.in_schedule(OnEnter(AppState::InGame)))

        .add_system(bird::game_start.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(menu::remove_menu_text.in_schedule(OnExit(AppState::MainMenu)))
        .add_systems(
            (
                bird::apply_bird_velocity,
                pipes::apply_pipes_velocity,
            )
            .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_systems(
            (
                bird::apply_bird_gravity,

                bird::move_bird,
                pipes::move_pipes,

                bird::bird_pipe_collisions,
                bird::bird_point_collisions,

                scoreboard::update_scoreboard,

                game_over::game_over,
            )
            .in_set(OnUpdate(AppState::InGame))
        )
        .run();
}

// Components & Resources used by more then one module outside main
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Debug, Clone, Copy, Eq, Default, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
