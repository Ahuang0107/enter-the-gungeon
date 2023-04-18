use bevy::prelude::*;
use bevy_3d_sprite::PbrSpriteBundle;
use std::f32::consts::PI;

/// 墙壁等倾斜等sprite
pub fn tilt_pbr_sprite(
    meshes: &mut ResMut<Assets<Mesh>>,
    size: Vec2,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    texture_atlas_handle: Handle<bevy_3d_sprite::TextureAtlas>,
    // pos.z表示的是倾斜的sprite的底部的z值，实际的z值会根据y的长度计算
    pos: Vec2,
    index: usize,
) -> PbrSpriteBundle {
    // 倾斜30度，所以需要显示的y是a的话，实际quad的y需要是2a/√3
    // TODO `3.0_f32.sqrt()`可以直接用常量
    let mesh_size = Vec2::new(size.x, (size.y * 2.0) / 3.0_f32.sqrt());
    PbrSpriteBundle {
        texture_atlas: texture_atlas_handle,
        sprite: bevy_3d_sprite::TextureAtlasSprite {
            index: Some(index),
            ..default()
        },
        mesh: meshes.add(Mesh::from(shape::Quad::new(mesh_size))),
        material: materials.add(StandardMaterial {
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            depth_bias: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(pos.x, pos.y, (size.y / 3.0_f32.sqrt()) / 2.0)
            .with_rotation(Quat::from_rotation_x(PI / 6.0)),
        ..default()
    }
}

pub fn plane_pbr_sprite(
    meshes: &mut ResMut<Assets<Mesh>>,
    size: Vec2,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    texture_atlas_handle: Handle<bevy_3d_sprite::TextureAtlas>,
    pos: Vec2,
    index: usize,
) -> PbrSpriteBundle {
    PbrSpriteBundle {
        texture_atlas: texture_atlas_handle,
        sprite: bevy_3d_sprite::TextureAtlasSprite {
            index: Some(index),
            ..default()
        },
        mesh: meshes.add(Mesh::from(shape::Quad::new(size))),
        material: materials.add(StandardMaterial {
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            depth_bias: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        ..default()
    }
}
