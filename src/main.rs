use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

mod character;
mod debug;
mod model;
mod tilemap;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AudioPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(bevy_3d_sprite::Sprite3dPlugin);
    app.insert_resource(ClearColor(Color::rgba_u8(3, 12, 14, 255)));
    app.add_startup_system(setup_camera)
        .add_startup_system(tilemap::setup)
        .add_startup_system(character::setup)
        .add_system(character::update_character_sprite)
        .add_system(character::play_character_sound)
        .add_system(character::character_move);

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera3dBundle {
        projection: OrthographicProjection { ..default() }.into(),
        transform: Transform::from_xyz(0.0, 0.0, 20.0).with_scale(Vec3::splat(0.05)),
        ..default()
    });
}
