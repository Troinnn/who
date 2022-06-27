use std::default::Default;

use crate::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuData {
    cur: usize,
    items: Vec<(u8, Entity)>,
}

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

    let play_item = TextBundle {
        style: Style {
            ..default()
        },
        text: Text::with_section(
            "-> Play",
            TextStyle {
                font: font.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    };
    let exit_item = TextBundle {
        style: Style {
            ..default()
        },
        text: Text::with_section(
            "Exit",
            TextStyle {
                font: font.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    };

    let mut its = Vec::new();

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }).with_children(|parent| {
        let pi = parent.spawn_bundle(play_item).id();
        let ei = parent.spawn_bundle(exit_item).id();
        its.push((0, pi));
        its.push((1, ei));
    });

    commands.insert_resource(MenuData {
        cur: 0,
        items: its,
    });
}

pub fn menu(
    mut state: ResMut<State<AppState>>,
    input: Res<Input<KeyCode>>,
    mut menu_data: ResMut<MenuData>,
    mut query: Query<&mut Transform, With<Text>>,
) {

    let mut text_transform = query.single_mut();

    if input.pressed(KeyCode::Up) {
        if menu_data.cur as isize - 1 < 0 {
            menu_data.cur = menu_data.items.len();
        }
        menu_data.cur -= 1;
    }

    if input.pressed(KeyCode::Down) {
        menu_data.cur += 1;
        if menu_data.cur > menu_data.items.len() - 1 {
            menu_data.cur = 0;
        }
    }


}
