use std::collections::HashMap;

use bevy::prelude::*;

use crate::resource::{ResourceCache, SCALE_RATIO};
use crate::utils;

pub fn setup(mut c: Commands, cache: Res<ResourceCache>) {
    let level = &cache.levels[0];
    let mut tilesets = HashMap::new();
    for tileset in cache.levels[0].tilesets.iter() {
        tilesets.insert(tileset.uuid.clone(), tileset.clone());
    }

    for room in level.rooms.iter() {
        let world_x = room.world_pos[0] * SCALE_RATIO;
        let world_y = room.world_pos[1] * SCALE_RATIO;
        c.spawn(SpatialBundle {
            transform: Transform {
                translation: Vec2::new(world_x, world_y).extend(1.5),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Room"))
        .with_children(|p| {
            // 添加墙壁
            p.spawn(SpatialBundle::default())
                .with_children(|p| {
                    for tile_group in room.walls.iter() {
                        let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                        for tile in tile_group.tiles.iter() {
                            let tile_info = tileset.tiles.get(&tile.index).unwrap();
                            let width = tile_info.size[0];
                            let height = tile_info.size[1];
                            p.spawn(utils::tile_wall_sprite(
                                cache.get_tilt_mesh((width, height)),
                                cache.get_material(&tile_group.tileset_uuid, tile.index),
                                tile.pos,
                                height as f32,
                            ))
                            .insert(Name::new("Wall"));
                        }
                    }
                })
                .insert(Name::new("Walls"));

            // 添加地板
            p.spawn(SpatialBundle::default())
                .with_children(|p| {
                    for tile_group in room.floors.iter() {
                        let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                        for tile in tile_group.tiles.iter() {
                            let tile_info = tileset.tiles.get(&tile.index).unwrap();
                            let width = tile_info.size[0];
                            let height = tile_info.size[1];
                            p.spawn(utils::plane_pbr_sprite(
                                cache.get_plane_mesh((width, height)),
                                cache.get_material(&tile_group.tileset_uuid, tile.index),
                                tile.pos,
                            ))
                            .insert(Name::new("Floor"));
                        }
                    }
                })
                .insert(Name::new("Floors"));

            // 添加天花板
            p.spawn(SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..default()
            })
            .with_children(|p| {
                for tile_group in room.roofs.iter() {
                    let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                    for tile in tile_group.tiles.iter() {
                        let tile_info = tileset.tiles.get(&tile.index).unwrap();
                        let width = tile_info.size[0];
                        let height = tile_info.size[1];
                        p.spawn(utils::plane_pbr_sprite(
                            cache.get_plane_mesh((width, height)),
                            cache.get_material(&tile_group.tileset_uuid, tile.index),
                            tile.pos,
                        ))
                        .insert(Name::new("Roof"));
                    }
                }
            })
            .insert(Name::new("Roofs"));

            // 添加灯光
            p.spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..default()
            })
            .with_children(|p| {
                for light in room.lights.iter() {
                    p.spawn(utils::point_light(light.pos, light.color))
                        .insert(Name::new("Light"));
                }
            })
            .insert(Name::new("Lights"));
        });
    }

    c.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20000.0,
            color: Color::rgba_u8(255, 172, 172, 172),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Global Light"));
}
