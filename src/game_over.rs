use bevy::prelude::*;

// Constants
const GAME_OVER_TEXT_SIZE: f32 = 56.;

// Components, Resources, Events
#[derive(Resource)]
pub struct GameOver(pub bool);

// Will eventually end the game when a collision event is
// detected
pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over: ResMut<GameOver>,
    collision_event: EventReader<super::bird::BirdCollisionEvent>,
    mut window: Query<&Window>,
) {
    if !collision_event.is_empty() && game_over.0 == false  {
        game_over.0 = true;

        let window = window.single();
        let text_x_position = (window.width() - GAME_OVER_TEXT_SIZE) / 2.;
        let text_y_position = (window.height() - GAME_OVER_TEXT_SIZE) / 2.;

        commands.spawn(
            TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: GAME_OVER_TEXT_SIZE,
                    color: super::TEXT_COLOR,
                }
            )
        .with_text_alignment(TextAlignment::Center)
        .with_style(
            Style {
                position: UiRect {
                    top: Val::Px(text_y_position), 
                    left: Val::Px(text_x_position),
                    ..default()

                },
                ..default()
            }
        ));
    }
}
