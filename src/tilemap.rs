use bevy::prelude::*;

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap_world = tilemap::TilemapWorld::from_ldtk("assets/level.ldtk").unwrap();

    let mut tileset_map: std::collections::HashMap<usize, Handle<TextureAtlas>> =
        std::collections::HashMap::new();
    for tileset in tilemap_world.tilesets.iter() {
        let texture_altlas = TextureAtlas::from_grid(
            asset_server.load(tileset.rel_path.as_str()),
            Vec2::splat(tileset.tile_grid_size as f32),
            tileset.c_w,
            tileset.c_h,
            None,
            None,
        );
        tileset_map.insert(tileset.uid, texture_atlases.add(texture_altlas));
    }

    for level in tilemap_world.levels.iter() {
        for layer in level.layers.iter() {
            let texture_altlas_handle = tileset_map.get(&layer.tileset_uid).unwrap();
            info!("{}", layer.name);
            layer.for_each(|(x, y, tile)| {
                if tile > 0 {
                    c.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: tile as usize - 1,
                            ..default()
                        },
                        texture_atlas: texture_altlas_handle.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x as f32 * 16.0,
                                y as f32 * -16.0,
                                match layer.name.as_str() {
                                    "WallTop" => 5.0,
                                    "WallTop_bg" => 4.0,
                                    "Wall" => 3.0,
                                    "FloorDecoration" => 2.0,
                                    "Floor" => 1.0,
                                    _ => 0.0,
                                },
                            ),
                            ..default()
                        },
                        ..default()
                    });
                }
            });
        }
    }
}
