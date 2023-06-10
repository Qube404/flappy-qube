/// A version of flappy bird

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

mod bird;
mod pipes;
mod camera;
mod game_over;
mod game_ui;

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
        .add_plugin(FrameTimeDiagnosticsPlugin)

        .add_state::<AppState>()
        
        .insert_resource(game_ui::scoreboard::Scoreboard { score: 0 })
        .insert_resource(game_ui::high_score::HighScore { highscore: 0 })
        .insert_resource(game_ui::fps::FpsSpawned(false))
        .insert_resource(game_ui::high_score::HighScoreSpawned(false))
        .insert_resource(ClearColor(BACKGROUND_COLOR))

        .add_event::<bird::BirdCollisionEvent>()

        .add_startup_system(bird::setup)
        .add_startup_system(pipes::setup)
        .add_startup_system(camera::setup)
        .add_startup_system(game_ui::setup)

        .add_system(game_ui::menu::setup.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(game_ui::menu::remove_menu_text.in_schedule(OnExit(AppState::MainMenu)))

        .add_system(game_ui::fps::setup.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(game_ui::high_score::setup.in_schedule(OnEnter(AppState::MainMenu))
            .after(game_ui::menu::setup)
        )

        .add_system(game_ui::scoreboard::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(game_ui::scoreboard::remove_scoreboard_text.in_schedule(OnExit(AppState::InGame)))

        .add_systems(
            (
                bird::game_start,
                bird::idle_bird_jump,
            ).in_set(OnUpdate(AppState::MainMenu))
        )
        .add_systems(
            (
                bird::apply_bird_velocity,
                bird::apply_bird_gravity,
                bird::rotate_bird,

                pipes::apply_pipes_velocity,

                game_ui::fps::update_fps,
            )
            .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_systems(
            (
                bird::move_bird,
                pipes::move_pipes,

                bird::bird_pipe_collisions,
                bird::bird_point_collisions,
                bird::bird_boundary_collisions,

                game_ui::scoreboard::update_scoreboard,
                game_ui::high_score::update_highscore,

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
