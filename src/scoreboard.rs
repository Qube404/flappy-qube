use bevy::prelude::*;

// Constants
const SCOREBOARD_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single();
    let text_x_position = (window.width() - SCOREBOARD_TEXT_SIZE) / 2.;
    let text_y_position = window.height() / 10.;

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_TEXT_SIZE,
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
        ),

        ScoreboardText,
    ));
}

// Components, Resources, Events
#[derive(Resource)]
pub struct Scoreboard {
    pub score: i128,
}

#[derive(Component)]
pub struct ScoreboardText;

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = scoreboard.score.to_string();
}
