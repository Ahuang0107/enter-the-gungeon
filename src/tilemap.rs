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
            p.spawn((
                Name::from("debug node"),
                PbrBundle {
                    mesh: cache.tile_debug_mesh.clone(),
                    material: cache.tile_world_debug_material.clone(),
                    ..default()
                },
            ));
            for room in level.rooms.iter() {
                let room_x = room.world_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
                let room_z = -room.world_pos[1] as f32 * GRID_SIZE * SCALE_RATIO * SQRT_2;
                p.spawn((
                    Name::new(room.display_name.clone()),
                    SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(room_x, 0.0, room_z),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Name::from("debug node"),
                        PbrBundle {
                            mesh: cache.tile_debug_mesh.clone(),
                            material: cache.tile_room_debug_material.clone(),
                            ..default()
                        },
                    ));
                    // 添加墙壁
                    p.spawn((
                        Name::new("Walls"),
                        SpatialBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    0.0,
                                    GRID_SIZE * SCALE_RATIO * SQRT_2,
                                    1.5 * GRID_SIZE * SCALE_RATIO * SQRT_2,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        for tile_group in room.walls.iter() {
                            let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                            for (grid_x, col) in tile_group.tiles.iter() {
                                for (grid_y, index) in col.iter() {
                                    let tile_info = tileset.tiles.get(index).unwrap();
                                    let width = tile_info.1[0] as u32;
                                    let height = tile_info.1[1] as u32;
                                    p.spawn((
                                        Name::new("Wall"),
                                        utils::tile_wall_sprite(
                                            cache.get_tile_mesh_sqrt2((width, height)),
                                            cache.get_tile_material(
                                                &tile_group.tileset_uuid,
                                                *index,
                                            ),
                                            [*grid_x as i32, *grid_y as i32],
                                            height,
                                        ),
                                    ));
                                }
                            }
                        }
                    });

                    // 添加地板
                    p.spawn((Name::new("Floors"), SpatialBundle::default()))
                        .with_children(|p| {
                            for tile_group in room.floors.iter() {
                                let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                                for (grid_x, col) in tile_group.tiles.iter() {
                                    for (grid_y, index) in col.iter() {
                                        let tile_info = tileset.tiles.get(index).unwrap();
                                        let width = tile_info.1[0] as u32;
                                        let height = tile_info.1[1] as u32;
                                        p.spawn((
                                            Name::new("Floor"),
                                            utils::tile_floor_sprite(
                                                cache.get_tile_mesh_sqrt2((width, height)),
                                                cache.get_tile_material(
                                                    &tile_group.tileset_uuid,
                                                    *index,
                                                ),
                                                [*grid_x as i32, *grid_y as i32],
                                            ),
                                        ));
                                    }
                                }
                            }
                        });

                    // 添加天花板
                    p.spawn((
                        Name::new("Roofs"),
                        SpatialBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    0.0,
                                    2.0 * GRID_SIZE * SCALE_RATIO * SQRT_2,
                                    2.0 * GRID_SIZE * SCALE_RATIO * SQRT_2,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        for tile_group in room.roofs.iter() {
                            let tileset = tilesets.get(&tile_group.tileset_uuid).unwrap();
                            for (grid_x, col) in tile_group.tiles.iter() {
                                for (grid_y, index) in col.iter() {
                                    let tile_info = tileset.tiles.get(index).unwrap();
                                    let width = tile_info.1[0] as u32;
                                    let height = tile_info.1[1] as u32;
                                    p.spawn((
                                        Name::new("Roof"),
                                        utils::tile_floor_sprite(
                                            cache.get_tile_mesh_sqrt2((width, height)),
                                            cache.get_tile_material(
                                                &tile_group.tileset_uuid,
                                                *index,
                                            ),
                                            [*grid_x as i32, *grid_y as i32],
                                        ),
                                    ));
                                }
                            }
                        }
                    });

                    // 添加灯光
                    p.spawn((
                        Name::new("Lights"),
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                0.0,
                                2.1 * GRID_SIZE * SCALE_RATIO * SQRT_2,
                                0.0,
                            ),
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        for light in room.lights.iter() {
                            p.spawn((
                                Name::new("Light"),
                                utils::point_light(light.pos, light.color),
                            ))
                            .with_children(|p| {
                                p.spawn(PbrBundle {
                                    mesh: cache.light_debug_mesh.clone(),
                                    material: cache.light_debug_material.clone(),
                                    ..default()
                                });
                            });
                        }
                    });
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
