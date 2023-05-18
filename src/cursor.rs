use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::res::SCALE_RATIO;

#[derive(Resource, Reflect, Default)]
pub struct ResCursor {
    screen_pos: Vec2,
    /// actual bevy world coordinates
    world_pos: Vec3,
}

impl ResCursor {
    pub fn get_world_pos(&self) -> Vec3 {
        self.world_pos
    }
    pub fn get_tilemap_pos(&self) -> [f32; 2] {
        [
            self.world_pos.x / SCALE_RATIO,
            self.world_pos.y / SCALE_RATIO,
        ]
    }
    pub fn get_ui_pos(&self) -> Vec2 {
        self.screen_pos
    }
}

pub struct CursorDetectPlugin;

impl Plugin for CursorDetectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResCursor::default());
        app.add_system(detect_cursor_position);
    }
}

pub fn detect_cursor_position(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<ResCursor>,
) {
    let primary_window = primary_window.get_single().unwrap();
    let (camera, global_transform) = camera_query.get_single().unwrap();
    if let Some(cursor_pos) = primary_window.cursor_position() {
        cursor.screen_pos = cursor_pos;
        if let Some(ray) = camera.viewport_to_world(global_transform, cursor_pos) {
            cursor.world_pos = ray.origin;
        }
    }
}
