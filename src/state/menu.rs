use std::collections::HashMap;
use std::default::Default;
use std::process::exit;
use bevy::app::AppExit;
use bevy::ecs::event::Events;
use bevy::pbr::LightEntity::Directional;

use crate::*;

const NORMAL_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
const HOVERED_BUTTON: Color = Color::rgb(0.95, 0.95, 0.95);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.95, 0.15);

pub struct MenuData {
    play_button: Entity,
    exit_button: Entity,
    selected: Entity,
}

#[derive(Component)]
pub struct MenuButton;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    let bb = ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut play_button_bundle = bb.clone();
    play_button_bundle.style.position = Rect {
        bottom: Val::Percent(53.0),
        left: Val::Percent(45.0),
        ..Default::default()
    };
    let play_button = commands.spawn_bundle(play_button_bundle)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            }).insert(MenuButton);
        }).insert(MenuButton).id();

    let mut exit_button_bundle = bb.clone();
    exit_button_bundle.style.position = Rect {
        bottom: Val::Percent(42.0),
        left: Val::Percent(45.0),
        ..Default::default()
    };
    let exit_button = commands.spawn_bundle(exit_button_bundle.clone())
        .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Exit",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
                Default::default(),
            ),
            ..Default::default()
        }).insert(MenuButton);
    }).insert(MenuButton).id();

    commands.insert_resource(MenuData {
        play_button,
        exit_button,
        selected: play_button
    });
}

pub fn menu(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut exit: ResMut<Events<AppExit>>,
    mut menu_data: ResMut<MenuData>,
    mut button_colors: Query<(Entity, &mut UiColor), With<Button>>,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut menu_buttons: Query<&mut Visibility, With<MenuButton>>,
) {
    let mut selected_button = None;
    let mut next_button = HashMap::new();
    next_button.insert(menu_data.play_button, menu_data.exit_button);
    next_button.insert(menu_data.exit_button, menu_data.play_button);

    let mut prev_button = HashMap::new();
    prev_button.insert(menu_data.play_button, menu_data.exit_button);
    prev_button.insert(menu_data.exit_button, menu_data.play_button);

    if input.just_pressed(KeyCode::Return) {
        selected_button = Some(menu_data.selected);
    }

    if input.just_pressed(KeyCode::Up) {
        menu_data.selected = *prev_button.get(&menu_data.selected).unwrap();
    }

    if input.just_pressed(KeyCode::Down) {
        menu_data.selected = *next_button.get(&menu_data.selected).unwrap();
    }

    //mouse interaction
    for (button_entity, interaction) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => selected_button = Some(button_entity),
            Interaction::Hovered => menu_data.selected = button_entity,
            _ => ()
        }
    }

    //keyboard interaction
    for (entity, mut color) in button_colors.iter_mut() {
        if entity == menu_data.selected {
            *color = HOVERED_BUTTON.into();
        } else {
            *color = NORMAL_BUTTON.into();
        }
    }

    if let Some(selected_button) = selected_button {
        if selected_button == menu_data.play_button {
            state.set(AppState::InGame).unwrap();
        }
        if selected_button == menu_data.exit_button {
            exit.send(AppExit);
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.play_button).despawn_recursive();
    commands.entity(menu_data.exit_button).despawn_recursive();
}