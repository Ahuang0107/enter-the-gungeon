use bevy::prelude::*;
use std::f32::consts::PI;

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.spawn(plane_pbr_texture(
        &mut meshes,
        Vec2::new(32.0, 32.0),
        &mut materials,
        asset_server.load("art/initial_map.png"),
        Vec3::new(0.0, 0.0, 1.5),
    ))
    .insert(Name::new("Ground"))
    .with_children(|p| {
        p.spawn(plane_pbr_texture(
            &mut meshes,
            Vec2::new(6.6, 15.1),
            &mut materials,
            asset_server.load("art/initial_stone_seat.png"),
            Vec3::new(0.0, 12.0, 1.6),
        ))
        .with_children(|p| {
            p.spawn(plane_pbr_texture(
                &mut meshes,
                Vec2::new(2.9, 6.7),
                &mut materials,
                asset_server.load("art/initial_stone_statue.png"),
                Vec3::new(0.0, 1.9, 1.8),
            ));
            p.spawn(plane_pbr_texture(
                &mut meshes,
                Vec2::new(2.6, 3.0),
                &mut materials,
                asset_server.load("art/initial_stone_base.png"),
                Vec3::new(0.0, -1.8, 1.7),
            ));
            p.spawn(plane_pbr_texture(
                &mut meshes,
                Vec2::new(7.8, 6.1),
                &mut materials,
                asset_server.load("art/initial_stone_seat_shadow.png"),
                Vec3::new(0.0, -4.9, 1.7),
            ));
        });
    });
    c.spawn(tilt_pbr_texture(
        &mut meshes,
        Vec2::new(6.4, 3.2),
        &mut materials,
        asset_server.load("art/wall_tests.png"),
        Vec3::new(-16.0 + 6.4 / 2.0, 17.6, 1.5),
    ))
    .insert(Name::new("Wall"));
    c.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Global Light"));
    c.spawn(point_light(
        Vec2::new(-16.0 + 6.4 / 2.0, 17.6),
        Color::rgba(1.0, 1.0, 1.0, 0.9),
    ))
    .insert(Name::new("Left Top Lamp Light"));
    c.spawn(point_light(
        Vec2::new(15.0, -15.0),
        Color::rgba(0.8, 0.8, 0.0, 0.9),
    ))
    .insert(Name::new("Right Top Lamp Light"));
    c.spawn(point_light(
        Vec2::new(-15.0, -15.0),
        Color::rgba(1.0, 0.5, 0.2, 0.9),
    ))
    .insert(Name::new("Right Bottom Lamp Light"));
    c.spawn(point_light(
        Vec2::new(-2.2, 3.4),
        Color::rgba(1.0, 1.0, 1.0, 0.9),
    ))
    .insert(Name::new("Center Light"));
}

fn tilt_pbr_texture(
    meshes: &mut ResMut<Assets<Mesh>>,
    size: Vec2,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    image: Handle<Image>,
    pos: Vec3,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.x,
            (size.y * 2.0) / 3.0_f32.sqrt(),
        )))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(image),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            depth_bias: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z + (size.y / 3.0_f32.sqrt()) / 2.0)
            .with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    }
}

fn plane_pbr_texture(
    meshes: &mut ResMut<Assets<Mesh>>,
    size: Vec2,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    image: Handle<Image>,
    pos: Vec3,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(size))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(image),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
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
        transform: Transform::from_xyz(pos.x, pos.y, 1.5 + (3.2 / 3.0_f32.sqrt()) / 2.0 + 0.5),
        ..default()
    }
}
