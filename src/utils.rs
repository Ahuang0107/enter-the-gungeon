use bevy::prelude::*;
use std::f32::consts::PI;

/// 墙壁等倾斜等sprite
pub fn tilt_pbr_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: Vec2,
) -> PbrBundle {
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(pos.x, pos.y, (3.2 / 3.0_f32.sqrt()) / 2.0)
            .with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    }
}

pub fn plane_pbr_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: Vec2,
) -> PbrBundle {
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        ..default()
    }
}

pub fn point_light(pos: Vec2, color: Color) -> PointLightBundle {
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
