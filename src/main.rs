use bevy::prelude::*;
use bevy::render::{RenderApp, RenderSet};

mod custom_material;
mod custom_mesh;
mod debug;
mod render_set_system;
mod tilemap;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(debug::DebugPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(tilemap::setup)
        .add_startup_system(custom_mesh::setup)
        .add_startup_system(custom_material::setup);

    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system(render_set_system::setup.in_set(RenderSet::ExtractCommands));

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}
