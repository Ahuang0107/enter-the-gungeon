use bevy::prelude::*;
use bevy::render::{RenderApp, RenderSet};

mod character;
mod custom_material;
mod custom_mesh;
mod debug;
mod render_set_system;
mod sprite_animation;
mod tilemap;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(debug::DebugPlugin)
        .add_plugin(sprite_animation::AnimationPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(tilemap::setup)
        .add_startup_system(custom_mesh::setup)
        .add_startup_system(custom_material::setup)
        .add_startup_system(character::setup)
        .add_system(character::update_character_sprite)
        .add_system(character::character_move);

    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system(render_set_system::setup.in_set(RenderSet::ExtractCommands));

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}
