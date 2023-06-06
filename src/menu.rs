use bevy::prelude::*;

// Constants
const MENU_TEXT_SIZE: f32 = 56.;

// Initial Setup
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((NodeBundle {
        style: Style {
            flex_basis: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    },
    MenuText,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                padding: UiRect {
                    top: Val::Percent(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Press Space or Mouse1 to start!",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: MENU_TEXT_SIZE,
                    color: super::TEXT_COLOR,
                }
            ));
        });
    });
}

#[derive(Component)]
pub struct MenuText;

pub fn remove_menu_text(
    mut commands: Commands,
    mut query: Query<Entity, With<MenuText>>
) {
    let menu_text = query.single_mut();

    commands
        .entity(menu_text)
        .despawn();
}
