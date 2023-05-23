use bevy::prelude::*;

use crate::res::Cache;

#[derive(Component)]
pub struct GunCardUi;

pub fn setup(mut c: Commands, cache: Res<Cache>) {
    c.spawn((
        Name::new("Gun Card"),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Auto, Val::Px(24.0), Val::Auto, Val::Px(8.0)),
                size: Size::new(Val::Px(90.0), Val::Px(58.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.get_card_image(1).clone(),
                ..default()
            },
            ..default()
        },
        GunCardUi,
    ));
}
