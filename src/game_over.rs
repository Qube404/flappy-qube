
use bevy::prelude::*;

pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_event: EventReader<super::CollisionEvent>,
) {
    if !collision_event.is_empty() {
        commands
            .spawn(
                TextBundle::from_section(
                    "Game Over",
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
