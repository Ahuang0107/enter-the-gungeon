use bevy::prelude::*;

use crate::res::Cache;

#[derive(Component)]
pub struct AmmoBorderTopUi;

#[derive(Component)]
pub struct AmmoBorderBottomUi;

#[allow(dead_code)]
#[derive(Component)]
pub struct AmmoUi {
    index: i32,
}

pub fn setup(mut c: Commands, cache: Res<Cache>) {
    let mut bottom = 8.0;
    c.spawn((
        Name::new("UI Ammo Border Bottom"),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(8.0), Val::Auto, Val::Px(bottom)),
                size: Size::new(Val::Px(14.0), Val::Px(10.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_ammo_border.clone(),
                flip_y: true,
                ..default()
            },
            ..default()
        },
        AmmoBorderBottomUi,
    ));
    bottom += 10.0;
    let ui_ammo_image = cache.get_ui_ammo_images("budget_revolver").0.clone();
    for i in 0..5 {
        c.spawn((
            Name::new("UI Ammo"),
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(Val::Auto, Val::Px(10.0), Val::Auto, Val::Px(bottom)),
                    size: Size::new(Val::Px(10.0), Val::Px(6.0)),
                    ..default()
                },
                image: UiImage {
                    texture: ui_ammo_image.clone(),
                    ..default()
                },
                ..default()
            },
            AmmoUi { index: i },
        ));
        bottom += 6.0;
    }
    c.spawn((
        Name::new("UI Ammo Border Top"),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(8.0), Val::Auto, Val::Px(bottom)),
                size: Size::new(Val::Px(14.0), Val::Px(10.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_ammo_border.clone(),
                ..default()
            },
            ..default()
        },
        AmmoBorderTopUi,
    ));
}
