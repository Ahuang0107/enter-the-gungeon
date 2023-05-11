use std::collections::HashMap;
use std::f32::consts::SQRT_2;

use bevy::prelude::*;

use crate::res::{Cache, GRID_SIZE, SCALE_RATIO};
use crate::utils;

pub fn setup(mut c: Commands, cache: Res<Cache>) {
    let level = &cache.levels[0];
    let mut tilesets = HashMap::new();
    for tileset in cache.levels[0].tilesets.iter() {
        tilesets.insert(tileset.uuid.clone(), tileset.clone());
    }

    c.spawn(SpatialBundle::default())
        .insert(Name::new("Rooms"))
        .with_children(|p| {
            for (index, room) in level.rooms.iter().enumerate() {
                let room_x = room.world_pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
                let room_y = room.world_pos[1] as f32 * SCALE_RATIO * GRID_SIZE * SQRT_2;
                p.spawn(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(room_x, 0.0, -room_y),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("Room{index:#02x?}")))
                .with_children(|p| {
                    // 添加墙壁
                    p.spawn(SpatialBundle::default())
                        .with_children(|p| {
                            for tile_group in room.walls.iter() {
                                let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                                for (grid_x, col) in tile_group.tiles.iter() {
                                    for (grid_y, index) in col.iter() {
                                        let tile_info = tileset.tiles.get(index).unwrap();
                                        let width = tile_info.1[0] as u32;
                                        let height = tile_info.1[1] as u32;
                                        p.spawn(utils::tile_wall_sprite(
                                            cache.get_tile_mesh((width, height)),
                                            cache.get_tile_material(
                                                &tile_group.tileset_uuid,
                                                *index,
                                            ),
                                            [*grid_x as i32, *grid_y as i32],
                                            height,
                                        ))
                                        .insert(Name::new("Wall"));
                                    }
                                }
                            }
                        })
                        .insert(Name::new("Walls"));

                    // 添加地板
                    p.spawn(SpatialBundle::default())
                        .with_children(|p| {
                            for tile_group in room.floors.iter() {
                                let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                                for (grid_x, col) in tile_group.tiles.iter() {
                                    for (grid_y, index) in col.iter() {
                                        let tile_info = tileset.tiles.get(index).unwrap();
                                        let width = tile_info.1[0] as u32;
                                        let height = tile_info.1[1] as u32;
                                        p.spawn(utils::tile_floor_sprite(
                                            cache.get_tile_mesh((width, height)),
                                            cache.get_tile_material(
                                                &tile_group.tileset_uuid,
                                                *index,
                                            ),
                                            [*grid_x as i32, *grid_y as i32],
                                        ))
                                        .insert(Name::new("Floor"));
                                    }
                                }
                            }
                        })
                        .insert(Name::new("Floors"));

                    // 添加天花板
                    p.spawn(SpatialBundle {
                        transform: Transform::from_xyz(
                            0.0,
                            32.0 * SQRT_2 * SCALE_RATIO,
                            32.0 * SQRT_2 * SCALE_RATIO,
                        ),
                        ..default()
                    })
                    .with_children(|p| {
                        for tile_group in room.roofs.iter() {
                            let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                            for (grid_x, col) in tile_group.tiles.iter() {
                                for (grid_y, index) in col.iter() {
                                    let tile_info = tileset.tiles.get(index).unwrap();
                                    let width = tile_info.1[0] as u32;
                                    let height = tile_info.1[1] as u32;
                                    p.spawn(utils::tile_floor_sprite(
                                        cache.get_tile_mesh((width, height)),
                                        cache.get_tile_material(&tile_group.tileset_uuid, *index),
                                        [*grid_x as i32, *grid_y as i32],
                                    ))
                                    .insert(Name::new("Roof"));
                                }
                            }
                        }
                    })
                    .insert(Name::new("Roofs"));

                    // 添加灯光
                    p.spawn(SpriteBundle {
                        transform: Transform::from_xyz(
                            0.0,
                            32.0 * SQRT_2 * SCALE_RATIO,
                            32.0 * SQRT_2 * SCALE_RATIO,
                        ),
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
        });

    c.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 13000.0,
            color: Color::rgba_u8(255, 172, 172, 172),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 10.0)
            .looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
        ..default()
    })
    .insert(Name::new("Global Light"));
}
