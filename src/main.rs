use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::state::AppState;

mod state;

const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_state(AppState::Menu)
        .add_system(fps_update_text)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(state::menu::setup))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(state::menu::menu))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(state::menu::cleanup_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(state::game::setup))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(state::game::game))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

#[derive(Component)]
struct FpsText;

fn fps_update_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}