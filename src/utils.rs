use std::f32::consts::PI;

use bevy::prelude::*;

use crate::resource::{GRID_SIZE, SCALE_RATIO};

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: [i32; 2],
    height: f32,
) -> PbrBundle {
    let x = pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let y = (pos[1] as f32 - 0.5) * SCALE_RATIO * GRID_SIZE;
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
    pos: [i32; 2],
) -> PbrBundle {
    let x = pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let y = pos[1] as f32 * SCALE_RATIO * GRID_SIZE;
    let z = 0.0;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}

pub fn point_light(pos: [u32; 3], color: [u8; 4]) -> PointLightBundle {
    let x = pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let y = pos[1] as f32 * SCALE_RATIO * GRID_SIZE;
    let z = pos[2] as f32 * SCALE_RATIO;
    let intensity = if z > 0.0 { 5000.0 } else { 1000.0 };
    PointLightBundle {
        point_light: PointLight {
            color: Color::rgba_u8(color[0], color[1], color[2], color[3]),
            intensity,
            range: 50.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}
