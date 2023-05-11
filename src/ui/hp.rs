use bevy::prelude::*;

use crate::res::{Cache, ResActor};

#[derive(Component)]
pub struct HpUi {
    first: u8,
    second: u8,
}

pub fn setup(mut c: Commands, actor: Res<ResActor>, cache: Res<Cache>) {
    let mut offset = 16.0;
    for i in (0..actor.get_cur_hp()).step_by(2) {
        c.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(offset), Val::Auto, Val::Px(16.0), Val::Auto),
                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.get_hp_image(0).clone(),
                ..default()
            },
            ..default()
        })
        .insert(HpUi {
            first: i + 1,
            second: i + 2,
        });
        offset += 32.0 + 2.0;
    }
}

pub fn update(actor: Res<ResActor>, cache: Res<Cache>, mut query: Query<(&mut UiImage, &HpUi)>) {
    let hp = actor.get_cur_hp();
    for (mut ui_image, hp_ui) in query.iter_mut() {
        if hp_ui.first > hp {
            ui_image.texture = cache.get_hp_image(2).clone();
        } else if hp_ui.second > hp {
            ui_image.texture = cache.get_hp_image(1).clone();
        }
    }
}
