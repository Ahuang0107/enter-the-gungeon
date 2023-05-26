use bevy::prelude::*;

use crate::res::{Cache, ResActor};
use crate::utils::{u16_to_chars, u8_to_chars};

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
    let status = actor.get_status();
    let origin = [6.0, 8.0];
    let mut offset_y = origin[1];
    // HP的UI
    {
        let size = [30.0, 26.0];
        let space_x = 2.0;
        let hp = status.get_cur_hp();
        assert_eq!(hp % 2, 0, "all actor default hp should multiple of 2");
        let hp = hp / 2;

        let mut offset_x = origin[0];
        for i in 0..hp {
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Px(offset_x),
                        Val::Auto,
                        Val::Px(offset_y),
                        Val::Auto,
                    ),
                    size: Size::new(Val::Px(size[0]), Val::Px(size[1])),
                    ..default()
                },
                image: UiImage {
                    texture: cache.get_hp_image(0).clone(),
                    ..default()
                },
                ..default()
            })
            .insert(HpUi {
                first: i * 2 + 1,
                second: i * 2 + 2,
            });
            offset_x += size[0] + space_x;
        }
        offset_y += size[1] + 2.0;
    }
    // Blank的UI
    {
        let size = [24.0, 24.0];
        let mut offset_x = origin[0];
        for i in 0..status.get_blanks() {
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Px(offset_x),
                        Val::Auto,
                        Val::Px(offset_y),
                        Val::Auto,
                    ),
                    size: Size::new(Val::Px(size[0]), Val::Px(size[1])),
                    ..default()
                },
                image: UiImage {
                    texture: cache.ui_blank_image.clone(),
                    ..default()
                },
                ..default()
            })
            .insert(BlankUi { index: i + 1 });
            offset_x += size[0] + 2.0;
        }
        offset_y += size[1] + 2.0;
    }
    // key和money
    {
        let mut offset_x = origin[0];
        let font_size = [14.0, 18.0];
        let font_offset_y = 5.0;
        // Key的UI
        {
            let size = [32.0, 28.0];
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Px(offset_x),
                        Val::Auto,
                        Val::Px(offset_y),
                        Val::Auto,
                    ),
                    size: Size::new(Val::Px(size[0]), Val::Px(size[1])),
                    ..default()
                },
                image: UiImage {
                    texture: cache.ui_key_image.clone(),
                    ..default()
                },
                ..default()
            })
            .insert(KeyUi);
            offset_x += size[0];
            for v in u8_to_chars(status.get_keys()) {
                c.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect::new(
                            Val::Px(offset_x),
                            Val::Auto,
                            Val::Px(offset_y + font_offset_y),
                            Val::Auto,
                        ),
                        size: Size::new(Val::Px(font_size[0]), Val::Px(font_size[1])),
                        ..default()
                    },
                    image: UiImage {
                        texture: cache.ui_ascii_font.get(v).clone(),
                        ..default()
                    },
                    ..default()
                })
                .insert(KeyCountUi);
                offset_x += font_size[0];
            }
        }
        offset_x += 5.0;
        // Money的UI
        {
            let size = [24.0, 28.0];
            c.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Px(offset_x),
                        Val::Auto,
                        Val::Px(offset_y),
                        Val::Auto,
                    ),
                    size: Size::new(Val::Px(size[0]), Val::Px(size[1])),
                    ..default()
                },
                image: UiImage {
                    texture: cache.ui_money_image.clone(),
                    ..default()
                },
                ..default()
            })
            .insert(MoneyUi);
            offset_x += size[0];
            for v in u16_to_chars(status.get_money()) {
                c.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect::new(
                            Val::Px(offset_x),
                            Val::Auto,
                            Val::Px(offset_y + font_offset_y),
                            Val::Auto,
                        ),
                        size: Size::new(Val::Px(font_size[0]), Val::Px(font_size[1])),
                        ..default()
                    },
                    image: UiImage {
                        texture: cache.ui_ascii_font.get(v).clone(),
                        ..default()
                    },
                    ..default()
                })
                .insert(MoneyCountUi);
                offset_x += font_size[0];
            }
        }
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
