use std::collections::HashMap;
use std::f32::consts::{PI, SQRT_2};

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::res::{GRID_SIZE, SCALE_RATIO};

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
    height: u32,
) -> PbrBundle {
    let x = relative_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
    let y = relative_pos[1] as f32 * GRID_SIZE * SCALE_RATIO;
    let z_offset = (height / 2) as f32 * SQRT_2 * SCALE_RATIO;
    let z = -y + z_offset;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}

pub fn tile_floor_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
) -> PbrBundle {
    let x = relative_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
    let y = relative_pos[1] as f32 * GRID_SIZE * SCALE_RATIO;
    let z = -y;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_x(-PI / 4.0)),
        ..default()
    }
}

pub fn point_light(pos: [u32; 3], color: [u8; 4]) -> PointLightBundle {
    let x = pos[0] as f32 * SCALE_RATIO * GRID_SIZE;
    let y = pos[1] as f32 * GRID_SIZE * SCALE_RATIO;
    let z = -y;
    PointLightBundle {
        point_light: PointLight {
            color: Color::rgba_u8(color[0], color[1], color[2], color[3]),
            intensity: 2000.0,
            range: 10.0,
            radius: 5.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..default()
    }
}

pub fn split_images<P>(path: P, tile_size: Vec2, columns: usize, rows: usize) -> HashMap<u8, Image>
where
    P: AsRef<std::path::Path>,
{
    let mut texture_atlas = HashMap::new();
    let mut dynamic_image = image::open(path).unwrap();
    let buffer = dynamic_image.as_mut_rgba8().unwrap();
    for y in 0..rows {
        for x in 0..columns {
            let cell = Vec2::new(x as f32, y as f32);
            let rect_min = tile_size * cell;
            let rect = Rect {
                min: rect_min,
                max: rect_min + tile_size,
            };
            let sub_buffer = image::imageops::crop(
                buffer,
                rect.min.x as u32,
                rect.min.y as u32,
                tile_size.x as u32,
                tile_size.y as u32,
            )
            .to_image();
            let sub_image = Image::new(
                Extent3d {
                    width: sub_buffer.width(),
                    height: sub_buffer.height(),
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                sub_buffer.into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            );
            texture_atlas.insert((y * columns + x) as u8, sub_image);
        }
    }
    texture_atlas
}

pub fn load_actor_sprite<P>(config_path: P, image_path: P) -> HashMap<String, Vec<Image>>
where
    P: AsRef<std::path::Path>,
{
    let mut result: HashMap<String, Vec<Image>> = HashMap::new();

    let config = aseprite::Output::from(config_path).unwrap();
    let mut dynamic_image = image::open(image_path).unwrap();
    let buffer = dynamic_image.as_mut_rgba8().unwrap();
    for item in config.frames {
        let tag = item.filename;
        let sub_buffer = image::imageops::crop(
            buffer,
            item.frame.x,
            item.frame.y,
            item.frame.w,
            item.frame.h,
        )
        .to_image();
        let sub_image = Image::new(
            Extent3d {
                width: sub_buffer.width(),
                height: sub_buffer.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            sub_buffer.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
        );
        if let Some(frames) = result.get_mut(&tag) {
            frames.push(sub_image);
        } else {
            result.insert(tag, vec![sub_image]);
        }
    }
    result
}
