use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use crate::cursor::ResCursor;
use crate::res::ResActor;
use crate::sprite_animation::{MaterialSprite, SpriteAnimation};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(ScreenDiagnosticsPlugin::default())
            .add_plugin(ScreenFrameDiagnosticsPlugin)
            .register_type::<MaterialSprite>()
            .register_type::<SpriteAnimation>()
            // .add_system(move_camera)
            .add_system(scroll_camera);
        {
            app.add_plugin(ResourceInspectorPlugin::<ResActor>::default());
            app.register_type::<ResActor>();
        }
        {
            app.add_plugin(ResourceInspectorPlugin::<ResCursor>::default());
            app.register_type::<ResCursor>();
        }
    }
}

#[allow(dead_code)]
fn move_camera(
    mut camera_current: Local<Vec2>,
    mut camera_target: Local<Vec2>,
    mut query_cameras: Query<&mut Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let speed = 1.0;

    if keyboard.pressed(KeyCode::Up) {
        camera_target.y += speed;
    }
    if keyboard.pressed(KeyCode::Down) {
        camera_target.y -= speed;
    }
    if keyboard.pressed(KeyCode::Left) {
        camera_target.x -= speed;
    }
    if keyboard.pressed(KeyCode::Right) {
        camera_target.x += speed;
    }

    let blend_ratio = 0.18;
    let movement = *camera_target - *camera_current;
    *camera_current += movement * blend_ratio;

    for mut camera_transform in query_cameras.iter_mut() {
        camera_transform.translation.x = camera_current.x;
        camera_transform.translation.y = camera_current.y;
    }
}

fn scroll_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if keyboard_input.pressed(KeyCode::LShift) {
        for ev in scroll_evr.iter() {
            for mut transform in query.iter_mut() {
                transform.scale += ev.y / 1000.0;
            }
        }
    }
}
