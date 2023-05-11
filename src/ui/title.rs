use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct TitleUi;

pub fn setup(mut c: Commands, asset_server: Res<AssetServer>) {
    c.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: Color::NONE.into(),
        ..default()
    })
    .insert(TitleUi)
    .with_children(|p| {
        p.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(10.0), Val::Auto, Val::Px(10.0)),
                ..default()
            },
            image: UiImage {
                texture: asset_server.load("art/ui/title_background.png"),
                ..default()
            },
            ..default()
        });

        p.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(186.0), Val::Auto, Val::Px(202.0)),
                ..default()
            },
            image: UiImage {
                texture: asset_server.load("art/ui/title_background_highlight.png"),
                ..default()
            },
            ..default()
        });
        p.spawn(ImageBundle {
            style: Style { ..default() },
            image: UiImage {
                texture: asset_server.load("art/ui/title_words_black_001.png"),
                ..default()
            },
            ..default()
        });
        p.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(183.0), Val::Auto, Val::Px(345.0)),
                ..default()
            },
            image: UiImage {
                texture: asset_server.load("art/ui/dragon_single.png"),
                ..default()
            },
            ..default()
        });
        let font_handle = asset_server.load("fonts/ThaleahFat.ttf");
        p.spawn(
            TextBundle::from_section(
                crate::VERSION,
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(25.0), Val::Auto, Val::Px(20.0)),
                ..default()
            }),
        );
        p.spawn(
            TextBundle::from_section(
                "Start",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(50.0), Val::Auto, Val::Auto, Val::Px(100.0)),
                ..default()
            }),
        );
    });
}

pub fn detect_start(mut next_state: ResMut<NextState<AppState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Loading);
    }
}

pub fn dismount(mut c: Commands, query: Query<Entity, With<TitleUi>>) {
    for entity in query.iter() {
        c.entity(entity).despawn_recursive();
    }
}
