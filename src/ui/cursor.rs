use bevy::prelude::*;

use crate::cursor::ResCursor;

#[derive(Component)]
pub struct CursorUi {
    size: [f32; 2],
}

impl CursorUi {
    fn get_width(&self) -> f32 {
        self.size[0]
    }
    fn get_height(&self) -> f32 {
        self.size[1]
    }
    fn get_width_offset(&self) -> f32 {
        self.size[0] / 2.0
    }
    fn get_height_offset(&self) -> f32 {
        self.size[1] / 2.0
    }
}

pub fn setup(mut c: Commands, cursor: Res<ResCursor>, asset_server: Res<AssetServer>) {
    let ui_pos = cursor.get_ui_pos();
    let cursor_ui = CursorUi { size: [22.0, 22.0] };
    c.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(
                Val::Px(ui_pos.x - cursor_ui.get_width_offset()),
                Val::Auto,
                Val::Auto,
                Val::Px(ui_pos.y - cursor_ui.get_height_offset()),
            ),
            size: Size::new(
                Val::Px(cursor_ui.get_width()),
                Val::Px(cursor_ui.get_height()),
            ),
            ..default()
        },
        image: UiImage {
            texture: asset_server.load("art/ui/cursor.png"),
            ..default()
        },
        ..default()
    })
    .insert(cursor_ui);
}

pub fn update(mut query: Query<(&mut Style, &CursorUi)>, cursor: Res<ResCursor>) {
    let ui_pos = cursor.get_ui_pos();
    for (mut style, cursor_ui) in query.iter_mut() {
        style.position.left = Val::Px(ui_pos.x - cursor_ui.get_width_offset());
        style.position.bottom = Val::Px(ui_pos.y - cursor_ui.get_height_offset());
    }
}
