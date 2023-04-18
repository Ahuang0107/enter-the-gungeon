use crate::model::RoomModel;
use crate::utils;
use bevy::prelude::*;

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlases: ResMut<Assets<bevy_3d_sprite::TextureAtlas>>,
) {
    let initial_room = RoomModel::initial();

    let wall_texture_atlas = texture_atlases.add(bevy_3d_sprite::TextureAtlas::from_grid(
        asset_server.load("art/wall.png"),
        Vec2::new(16.0, 32.0),
        12,
        1,
    ));
    let roof_texture_atlas = texture_atlases.add(bevy_3d_sprite::TextureAtlas::from_grid(
        asset_server.load("art/roof.png"),
        Vec2::new(16.0, 16.0),
        6,
        4,
    ));
    c.spawn(SpatialBundle {
        transform: Transform {
            translation: Vec2::from(initial_room.translation()).extend(1.5),
            ..default()
        },
        ..default()
    })
    .with_children(|p| {
        p.spawn(plane_pbr_texture(
            &mut meshes,
            Vec2::new(320.0 / 10.0, 320.0 / 10.0),
            &mut materials,
            asset_server.load("art/initial_map.png"),
            Vec3::ZERO,
            0.0,
        ))
        .insert(Name::new("Ground"))
        .with_children(|p| {
            p.spawn(plane_pbr_texture(
                &mut meshes,
                Vec2::new(6.6, 15.1),
                &mut materials,
                asset_server.load("art/initial_stone_seat.png"),
                Vec3::new(0.0, 12.0, 0.0),
                1.0,
            ))
            .with_children(|p| {
                p.spawn(plane_pbr_texture(
                    &mut meshes,
                    Vec2::new(2.9, 6.7),
                    &mut materials,
                    asset_server.load("art/initial_stone_statue.png"),
                    Vec3::new(0.0, 1.9, 0.0),
                    3.0,
                ));
                p.spawn(plane_pbr_texture(
                    &mut meshes,
                    Vec2::new(2.6, 3.0),
                    &mut materials,
                    asset_server.load("art/initial_stone_base.png"),
                    Vec3::new(0.0, -1.8, 0.0),
                    2.0,
                ));
                p.spawn(plane_pbr_texture(
                    &mut meshes,
                    Vec2::new(7.8, 6.1),
                    &mut materials,
                    asset_server.load("art/initial_stone_seat_shadow.png"),
                    Vec3::new(0.0, -4.9, 0.0),
                    1.0,
                ));
            });
        });
        // 添加墙壁
        p.spawn(SpatialBundle::default())
            .with_children(|p| {
                for wall_model in initial_room.walls.iter() {
                    p.spawn(utils::tilt_pbr_sprite(
                        &mut meshes,
                        Vec2::new(16.0 / 10.0, 32.0 / 10.0),
                        &mut materials,
                        wall_texture_atlas.clone(),
                        Vec2::from(wall_model.wall_translation()),
                        0,
                    ))
                    .insert(Name::new("Wall"));
                }
            })
            .insert(Name::new("Walls"));
        // 添加天花板
        p.spawn(SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        })
        .with_children(|p| {
            for roof_model in initial_room.roofs.iter() {
                p.spawn(utils::plane_pbr_sprite(
                    &mut meshes,
                    Vec2::new(16.0 / 10.0, 16.0 / 10.0),
                    &mut materials,
                    roof_texture_atlas.clone(),
                    Vec2::from(roof_model.translation()),
                    roof_model.sprite_index(),
                ))
                .insert(Name::new("Roof"));
            }
        })
        .insert(Name::new("Roofs"));
        // 添加灯光
        p.spawn(SpriteBundle::default())
            .with_children(|p| {
                for light_model in initial_room.lights.iter() {
                    p.spawn(point_light(
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

fn plane_pbr_texture(
    meshes: &mut ResMut<Assets<Mesh>>,
    size: Vec2,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    image: Handle<Image>,
    pos: Vec3,
    depth_bias: f32,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(size))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(image),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            depth_bias,
            ..default()
        }),
        transform: Transform {
            translation: pos,
            ..default()
        },
        ..default()
    }
}

fn point_light(pos: Vec2, color: Color) -> PointLightBundle {
    PointLightBundle {
        point_light: PointLight {
            color,
            intensity: 5000.0,
            range: 15.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(pos.x, pos.y, (3.2 / 3.0_f32.sqrt()) / 2.0 + 0.5),
        ..default()
    }
}
