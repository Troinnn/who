use crate::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(""),
       ..Default::default()
    });
}

pub fn game(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();
    }
}
