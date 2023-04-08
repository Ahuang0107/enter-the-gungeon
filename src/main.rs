use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

mod character;
mod debug;
mod sprite_animation;
mod tilemap;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AudioPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(sprite_animation::AnimationPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(tilemap::setup)
        .add_startup_system(character::setup)
        .add_system(character::update_character_sprite)
        .add_system(character::play_character_sound)
        .add_system(character::character_move);

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}
