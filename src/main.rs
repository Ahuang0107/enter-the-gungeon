use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    c.spawn(Camera2dBundle::default());

    let texture_altlas = TextureAtlas::from_grid(
        asset_server.load("art/floor.png"),
        Vec2::new(16.0, 16.0),
        10,
        2,
        None,
        None,
    );
    let texture_altlas_handle = texture_atlases.add(texture_altlas);

    #[rustfmt::skip]
    let level = [
        1, 1, 1, 1, 1, 1, 1, 1, 1,
        0, 0, 0, 0, 0, 0, 1, 0, 0,
        1, 0, 0, 0, 0, 0, 1, 0, 1,
        1, 0, 0, 0, 0, 1, 1, 0, 1,
        1, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 0, 1, 0, 0, 0, 0, 0, 1,
        1, 0, 1, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    for x in 0..9 {
        for y in 0..8 {
            let index = y * 9 + x;
            let tile = level[index];
            if tile != 1 {
                c.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    texture_atlas: texture_altlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * 16.0, y as f32 * 16.0, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
