use bevy::prelude::*;
use std::f32::consts::PI;

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32.0, 32.0)))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("art/initial_map.png")),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 1.5),
        ..default()
    });
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            6.4,
            (3.2 * 2.0) / 3.0_f32.sqrt(),
        )))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("art/wall_tests.png")),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            depth_bias: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(-16.0 + 6.4 / 2.0, 17.6, 1.5 + (3.2 / 3.0_f32.sqrt()) / 2.0)
            .with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    });
    c.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        ..default()
    });
    c.spawn(point_light(
        Vec2::new(-16.0 + 6.4 / 2.0, 17.6),
        Color::rgba(1.0, 1.0, 1.0, 0.9),
    ));
    c.spawn(point_light(
        Vec2::new(15.0, -15.0),
        Color::rgba(0.8, 0.8, 0.0, 0.9),
    ));
    c.spawn(point_light(
        Vec2::new(-15.0, -15.0),
        Color::rgba(1.0, 0.5, 0.2, 0.9),
    ));
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
