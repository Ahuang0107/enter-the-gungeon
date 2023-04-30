use std::f32::consts::PI;

use bevy::prelude::*;

use crate::resource::SCALE_RATIO;

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: [f32; 2],
    height: f32,
) -> PbrBundle {
    let x = pos[0] * SCALE_RATIO;
    let y = pos[1] * SCALE_RATIO;
    let z = (height * SCALE_RATIO / 3.0_f32.sqrt()) / 2.0;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    }
}

/// floor | roof tile
pub fn plane_pbr_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: [f32; 2],
) -> PbrBundle {
    let x = pos[0] * SCALE_RATIO;
    let y = pos[1] * SCALE_RATIO;
    let z = 0.0;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}

pub fn point_light(pos: [f32; 2], color: [u8; 4]) -> PointLightBundle {
    let x = pos[0] * SCALE_RATIO;
    let y = pos[1] * SCALE_RATIO;
    PointLightBundle {
        point_light: PointLight {
            color: Color::rgba_u8(color[0], color[1], color[2], color[3]),
            intensity: 5000.0,
            range: 50.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(x, y, 0.0),
        ..default()
    }
}
