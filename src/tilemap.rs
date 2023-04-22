use crate::model::RoomModel;
use crate::resource::ResourceCache;
use crate::utils;
use bevy::prelude::*;

pub fn setup(mut c: Commands, cache: Res<ResourceCache>) {
    let initial_room = RoomModel::initial();

    c.spawn(SpatialBundle {
        transform: Transform {
            translation: Vec2::from(initial_room.translation()).extend(1.5),
            ..default()
        },
        ..default()
    })
    .with_children(|p| {
        // 添加墙壁
        p.spawn(SpatialBundle::default())
            .with_children(|p| {
                for wall_model in initial_room.walls.iter() {
                    p.spawn(utils::tilt_pbr_sprite(
                        cache.tile_16_deg_30(),
                        cache.get_material("Wall", 0),
                        Vec2::from(wall_model.wall_translation()),
                    ))
                    .insert(Name::new("Wall"));
                }
            })
            .insert(Name::new("Walls"));

        // 添加地板
        p.spawn(SpatialBundle::default())
            .with_children(|p| {
                for (src, models) in initial_room.floor.iter() {
                    for model in models {
                        p.spawn(utils::plane_pbr_sprite(
                            cache.tile_16(),
                            cache.get_material(src, model.sprite_index()),
                            Vec2::from(model.translation()),
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
            for roof_model in initial_room.roofs.iter() {
                p.spawn(utils::plane_pbr_sprite(
                    cache.tile_16(),
                    cache.get_material("Roof", roof_model.sprite_index()),
                    Vec2::from(roof_model.translation()),
                ))
                .insert(Name::new("Roof"));
            }
        })
        .insert(Name::new("Roofs"));

        // 添加灯光
        p.spawn(SpriteBundle::default())
            .with_children(|p| {
                for light_model in initial_room.lights.iter() {
                    p.spawn(utils::point_light(
                        Vec2::from(light_model.translation()),
                        Color::rgba_u8(
                            light_model.color[0],
                            light_model.color[1],
                            light_model.color[2],
                            light_model.color[3],
                        ),
                    ))
                    .insert(Name::new("Light"));
                }
            })
            .insert(Name::new("Lights"));
    });
}
