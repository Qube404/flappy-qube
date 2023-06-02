use bevy::prelude::*;

// Scoreboard properties
const SCOREBOARD_TOP_PADDING: Val = Val::Px(500.);
const SCOREBOARD_LEFT_PADDING: Val = Val::Px(500.);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Score
    commands
        .spawn((
            TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 56.0,
                    color: super::TEXT_COLOR,
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
