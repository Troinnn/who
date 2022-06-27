use bevy::prelude::*;

use crate::state::AppState;

mod state;

const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(state::menu::setup))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(state::menu::menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(state::game::setup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}