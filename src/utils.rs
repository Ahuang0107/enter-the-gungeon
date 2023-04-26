use std::f32::consts::PI;

use bevy::prelude::*;

use crate::resource::{SCALE_RATIO, TILE_WALL_HEIGHT_PX};

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    pos: Vec2,
) -> PbrBundle {
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(
            pos.x,
            pos.y,
            (SCALE_RATIO * TILE_WALL_HEIGHT_PX / 3.0_f32.sqrt()) / 2.0,
        )
        .with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    }
}

/// floor | roof tile
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
            intensity: 1000.0,
            range: 50.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(pos.x, pos.y, 4.0),
        ..default()
    }
}
