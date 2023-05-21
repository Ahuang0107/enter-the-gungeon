use std::collections::HashMap;

use bevy::prelude::*;

use crate::res::{Cache, GRID_SIZE, GRID_SIZE_HALF, SCALE_RATIO};
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
            for room in level.rooms.iter() {
                let room_x = room.world_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
                let room_y = room.world_pos[1] as f32 * GRID_SIZE * SCALE_RATIO;
                let room_z = -room_y;
                p.spawn(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(room_x, room_y, room_z),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(room.display_name.clone()))
                .with_children(|p| {
                    // 添加墙壁
                    p.spawn(SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, -GRID_SIZE_HALF * SCALE_RATIO, 0.0),
                            ..default()
                        },
                        ..default()
                    })
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
                                        cache.get_tile_material(&tile_group.tileset_uuid, *index),
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
                    p.spawn(SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, -10.0 * SCALE_RATIO),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|p| {
                        for tile_group in room.floors.iter() {
                            let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                            for (grid_x, col) in tile_group.tiles.iter() {
                                for (grid_y, index) in col.iter() {
                                    let tile_info = tileset.tiles.get(index).unwrap();
                                    let width = tile_info.1[0] as u32;
                                    let height = tile_info.1[1] as u32;
                                    p.spawn(utils::tile_floor_sprite(
                                        cache.get_tile_mesh_sqrt2((width, height)),
                                        cache.get_tile_material(&tile_group.tileset_uuid, *index),
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
                            0.0,
                            // move roof more top to make sure actor and bullet will under roof
                            (GRID_SIZE + 20.0) * 2.0 * SCALE_RATIO,
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
                                        cache.get_tile_mesh_sqrt2((width, height)),
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
                        transform: Transform::from_xyz(0.0, 0.0, GRID_SIZE * 2.0 * SCALE_RATIO),
                        ..default()
                    })
                    .with_children(|p| {
                        for light in room.lights.iter() {
                            p.spawn(utils::point_light(light.pos, light.color))
                                .insert(Name::new("Light"))
                                .with_children(|p| {
                                    p.spawn(PbrBundle {
                                        mesh: cache.light_debug_mesh.clone(),
                                        material: cache.light_debug_material.clone(),
                                        ..default()
                                    });
                                });
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
