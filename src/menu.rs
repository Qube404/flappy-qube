use bevy::prelude::*;

// Constants
const MENU_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single();
    let text_x_position = (window.width() - MENU_TEXT_SIZE) / 2.;
    let text_y_position = window.height() / 2. - MENU_TEXT_SIZE;

    commands.spawn((
        TextBundle::from_section(
            "Press Space or Mouse1 to start!",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: MENU_TEXT_SIZE,
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

        MenuText,
    ));
}

#[derive(Component)]
struct MenuText;
