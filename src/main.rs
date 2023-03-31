use bevy::prelude::*;
use bevy::render::{RenderApp, RenderSet};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod custom_material;
mod custom_mesh;
mod render_set_system;
mod tilemap;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ScreenDiagnosticsPlugin::default())
        .add_plugin(ScreenFrameDiagnosticsPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(tilemap::setup)
        .add_startup_system(custom_mesh::setup)
        .add_startup_system(custom_material::setup)
        .add_system(system_move_camera);

    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system(render_set_system::setup.in_set(RenderSet::ExtractCommands));

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}

fn system_move_camera(
    mut camera_current: Local<Vec2>,
    mut camera_target: Local<Vec2>,
    mut query_cameras: Query<&mut Transform, With<Camera2d>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let speed = 10.0;

    if keyboard.pressed(KeyCode::W) {
        camera_target.y += speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera_target.y -= speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera_target.x -= speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera_target.x += speed;
    }

    // Smooth camera.
    let blend_ratio = 0.18;
    let movement = *camera_target - *camera_current;
    *camera_current += movement * blend_ratio;

    // Update all sprite cameras.
    for mut camera_transform in query_cameras.iter_mut() {
        camera_transform.translation.x = camera_current.x;
        camera_transform.translation.y = camera_current.y;
    }
}
