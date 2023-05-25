use bevy::prelude::*;

use crate::res::{Cache, ResActor};

#[derive(Component)]
pub struct HpUi {
    first: u8,
    second: u8,
}

#[derive(Component)]
pub struct BlankUi {
    index: u8,
}

#[derive(Component)]
pub struct KeyUi;

#[derive(Component)]
pub struct KeyCountUi;

#[derive(Component)]
pub struct MoneyUi;

#[derive(Component)]
pub struct MoneyCountUi;

/// 整个status的ui与左侧间隔3px，顶部间隔4px
/// 其中每个元素上下左右都间隔1px
pub fn setup(mut c: Commands, actor: Res<ResActor>, cache: Res<Cache>) {
    // HP的UI
    {
        let mut offset = 6.0;
        for i in (0..actor.get_status().get_cur_hp()).step_by(2) {
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(Val::Px(offset), Val::Auto, Val::Px(8.0), Val::Auto),
                    size: Size::new(Val::Px(30.0), Val::Px(26.0)),
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
            offset += 30.0 + 2.0;
        }
    }
    // Blank的UI
    {
        let mut offset = 6.0;
        for i in 0..actor.get_status().get_blanks() {
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(Val::Px(offset), Val::Auto, Val::Px(35.0), Val::Auto),
                    size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                    ..default()
                },
                image: UiImage {
                    texture: cache.ui_blank_image.clone(),
                    ..default()
                },
                ..default()
            })
            .insert(BlankUi { index: i + 1 });
            offset += 24.0 + 2.0;
        }
    }
    // Key的UI
    {
        c.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(6.0), Val::Auto, Val::Px(60.0), Val::Auto),
                size: Size::new(Val::Px(32.0), Val::Px(28.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_key_image.clone(),
                ..default()
            },
            ..default()
        })
        .insert(KeyUi);
        c.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(38.0), Val::Auto, Val::Px(68.0), Val::Auto),
                size: Size::new(Val::Px(14.0), Val::Px(18.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_ascii_font.get('1').clone(),
                ..default()
            },
            ..default()
        })
        .insert(KeyCountUi);
    }
    // Money的UI
    {
        c.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(66.0), Val::Auto, Val::Px(64.0), Val::Auto),
                size: Size::new(Val::Px(24.0), Val::Px(24.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_money_image.clone(),
                ..default()
            },
            ..default()
        })
        .insert(MoneyUi);
        c.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(92.0), Val::Auto, Val::Px(68.0), Val::Auto),
                size: Size::new(Val::Px(14.0), Val::Px(18.0)),
                ..default()
            },
            image: UiImage {
                texture: cache.ui_ascii_font.get('0').clone(),
                ..default()
            },
            ..default()
        })
        .insert(MoneyCountUi);
    }
}

pub fn update(actor: Res<ResActor>, cache: Res<Cache>, mut query: Query<(&mut UiImage, &HpUi)>) {
    let hp = actor.get_status().get_cur_hp();
    for (mut ui_image, hp_ui) in query.iter_mut() {
        if hp_ui.first > hp {
            ui_image.texture = cache.get_hp_image(2).clone();
        } else if hp_ui.second > hp {
            ui_image.texture = cache.get_hp_image(1).clone();
        }
    }
}
