use bevy::prelude::*;

// Constants
const GAME_OVER_TEXT_SIZE: f32 = 56.;

// Will eventually end the game when a collision event is
// detected
pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_event: EventReader<super::bird::BirdCollisionEvent>,
    window: Query<&Window>,
) {
    if !collision_event.is_empty() {
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
