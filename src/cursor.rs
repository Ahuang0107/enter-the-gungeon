use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::res::SCALE_RATIO;
use crate::utils;
use crate::CAMERA_FAR;

#[derive(Resource, Reflect, Default)]
pub struct ResCursor {
    screen_pos: Vec2,
    camera_pos: Vec3,
    /// actual bevy world coordinates
    world_pos: Vec3,
    tilemap_pos: [f32; 2],
}

impl ResCursor {
    pub fn set_screen_pos(&mut self, pos: Vec2) {
        self.screen_pos = pos;
    }
    pub fn set_camera_pos(&mut self, pos: Vec3) {
        self.camera_pos = pos;
    }
    pub fn set_world_pos(&mut self, pos: Vec3) {
        self.world_pos = pos;
        self.tilemap_pos = [
            self.world_pos.x / SCALE_RATIO,
            -(self.world_pos.y - CAMERA_FAR) / SQRT_2 / SCALE_RATIO,
        ];
    }
    pub fn update_tilemap_pos(&mut self) {
        // 计算得到当前相机映射到到tilemap中到位置
        let camera_tilemap_pos = utils::camera_pos_to_tilemap_pos(self.camera_pos);
        // 计算得到当前光标相对相机位置到偏移量
        let cursor_world_pos_offset = self.world_pos - self.camera_pos;
        // 计算得到偏移量对应到tilemap的值
        let cursor_tilemap_pos_offset = [
            cursor_world_pos_offset.x / SCALE_RATIO,
            -(cursor_world_pos_offset.z * SQRT_2) / SCALE_RATIO,
        ];
        self.tilemap_pos = [
            camera_tilemap_pos[0] + cursor_tilemap_pos_offset[0],
            camera_tilemap_pos[1] + cursor_tilemap_pos_offset[1],
        ];
    }
    pub fn get_world_pos(&self) -> Vec3 {
        self.world_pos
    }
    pub fn get_tilemap_pos(&self) -> [f32; 2] {
        self.tilemap_pos
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
        cursor.set_screen_pos(cursor_pos);
        cursor.set_camera_pos(global_transform.translation());
        if let Some(ray) = camera.viewport_to_world(global_transform, cursor_pos) {
            cursor.set_world_pos(ray.origin);
            cursor.update_tilemap_pos();
        }
    }
}
