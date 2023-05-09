use std::f32::consts::{PI, SQRT_2};

use bevy::prelude::*;

use crate::resource::{GRID_SIZE, SCALE_RATIO};

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
    height: u32,
) -> PbrBundle {
    let x = relative_pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let z =
        ((-relative_pos[1] as f32 * GRID_SIZE + (3 * height / 4) as f32) * SQRT_2) * SCALE_RATIO;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, (height / 2) as f32 * SQRT_2 * SCALE_RATIO, z),
        ..default()
    }
}

pub fn tile_floor_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
) -> PbrBundle {
    let x = relative_pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let z = -(((relative_pos[1] as f32 * GRID_SIZE) * SQRT_2) * SCALE_RATIO);
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, 0.0, z).with_rotation(Quat::from_rotation_x(-PI / 2.0)),
        ..default()
    }
}

pub fn point_light(pos: [u32; 3], color: [u8; 4]) -> PointLightBundle {
    let x = pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let z = -(pos[1] as f32 * SCALE_RATIO * GRID_SIZE * SQRT_2);
    PointLightBundle {
        point_light: PointLight {
            color: Color::rgba_u8(color[0], color[1], color[2], color[3]),
            intensity: 2000.0,
            range: 10.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(x, 0.0, z),
        ..default()
    }
}
