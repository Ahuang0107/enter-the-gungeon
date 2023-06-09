use std::collections::HashMap;
use std::f32::consts::{PI, SQRT_2};

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::res::{GRID_SIZE, SCALE_RATIO};
use crate::CAMERA_FAR;

/// wall tile
pub fn tile_wall_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
    _: u32,
) -> PbrBundle {
    let x = relative_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
    let z = -relative_pos[1] as f32 * GRID_SIZE * SCALE_RATIO * SQRT_2;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, 0.0, z),
        ..default()
    }
}

pub fn tile_floor_sprite(
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    relative_pos: [i32; 2],
) -> PbrBundle {
    let x = relative_pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
    let z = -relative_pos[1] as f32 * GRID_SIZE * SCALE_RATIO * SQRT_2;
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(x, 0.0, z).with_rotation(Quat::from_rotation_x(-PI / 2.0)),
        ..default()
    }
}

pub fn point_light(pos: [u32; 3], color: [u8; 4]) -> PointLightBundle {
    let x = pos[0] as f32 * GRID_SIZE * SCALE_RATIO;
    let z = -(pos[1] as f32) * GRID_SIZE * SCALE_RATIO * SQRT_2;
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

#[derive(Default, Debug, Copy, Clone)]
pub struct MoveDirection {
    pub x: MoveDirectionX,
    pub y: MoveDirectionY,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum MoveDirectionX {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum MoveDirectionY {
    Up,
    Down,
    #[default]
    None,
}

impl MoveDirection {
    pub fn detect_key(&mut self, keyboard: &Input<KeyCode>) {
        if keyboard.pressed(KeyCode::W) {
            self.up()
        }
        if keyboard.pressed(KeyCode::S) {
            self.down()
        }
        if keyboard.pressed(KeyCode::A) {
            self.left()
        }
        if keyboard.pressed(KeyCode::D) {
            self.right()
        }
    }
    fn up(&mut self) {
        match self.y {
            MoveDirectionY::None => {
                self.y = MoveDirectionY::Up;
            }
            MoveDirectionY::Down => {
                self.y = MoveDirectionY::None;
            }
            _ => {}
        }
    }
    fn down(&mut self) {
        match self.y {
            MoveDirectionY::None => {
                self.y = MoveDirectionY::Down;
            }
            MoveDirectionY::Up => {
                self.y = MoveDirectionY::None;
            }
            _ => {}
        }
    }
    fn left(&mut self) {
        match self.x {
            MoveDirectionX::None => {
                self.x = MoveDirectionX::Left;
            }
            MoveDirectionX::Right => {
                self.x = MoveDirectionX::None;
            }
            _ => {}
        }
    }
    fn right(&mut self) {
        match self.x {
            MoveDirectionX::None => {
                self.x = MoveDirectionX::Right;
            }
            MoveDirectionX::Left => {
                self.x = MoveDirectionX::None;
            }
            _ => {}
        }
    }
}

/// 读取并按照行列切割图片
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

/// 读取并按照行列切割图片
pub fn split_images_to_vec<P>(path: P, tile_size: Vec2, columns: usize, rows: usize) -> Vec<Image>
where
    P: AsRef<std::path::Path>,
{
    let mut texture_atlas = vec![];
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
            texture_atlas.push(sub_image);
        }
    }
    texture_atlas
}

pub fn load_actor_sprite<P>(config_path: P, image_path: P) -> HashMap<String, Vec<Image>>
where
    P: AsRef<std::path::Path>,
{
    let mut result: HashMap<String, Vec<Image>> = HashMap::new();

    let config = serde_aseprite::AsepriteDate::from(config_path).unwrap();
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

pub fn u8_to_chars(value: u8) -> Vec<char> {
    let result = if value < 10 {
        vec![(value + b'0') as char]
    } else if value < 100 {
        let tens_digit = value / 10;
        let digit = value % 10;
        vec![(tens_digit + b'0') as char, (digit + b'0') as char]
    } else {
        let hundreds_digit = value / 100;
        let tens_digit = (value % 100) / 10;
        let digit = value % 10;
        vec![
            (hundreds_digit + b'0') as char,
            (tens_digit + b'0') as char,
            (digit + b'0') as char,
        ]
    };
    debug!("convert u8 {value} to {result:?}");
    return result;
}

pub fn u16_to_chars(value: u16) -> Vec<char> {
    let result = if value < 10 {
        vec![(value as u8 + b'0') as char]
    } else if value < 100 {
        let tens_digit = value / 10;
        let digit = value % 10;
        vec![
            (tens_digit as u8 + b'0') as char,
            (digit as u8 + b'0') as char,
        ]
    } else {
        let hundreds_digit = value / 100;
        let tens_digit = (value % 100) / 10;
        let digit = value % 10;
        vec![
            (hundreds_digit as u8 + b'0') as char,
            (tens_digit as u8 + b'0') as char,
            (digit as u8 + b'0') as char,
        ]
    };
    debug!("convert u16 {value:?} to {result:?}");
    return result;
}

pub fn tilemap_pos_to_xy_pos(tilemap_pos: [f32; 2]) -> Vec3 {
    let x = tilemap_pos[0] as f32 * SCALE_RATIO;
    let z = -tilemap_pos[1] as f32 * SCALE_RATIO * SQRT_2;
    Vec3::new(x, 0.0, z)
}

pub fn tilemap_pos_to_camera_pos(tilemap_pos: [f32; 2]) -> Vec3 {
    let mut actual_pos = tilemap_pos_to_xy_pos(tilemap_pos);
    actual_pos.z += CAMERA_FAR;
    actual_pos.y += CAMERA_FAR;
    actual_pos
}

pub fn camera_pos_to_tilemap_pos(camera_pos: Vec3) -> [f32; 2] {
    [
        camera_pos.x / SCALE_RATIO,
        -(camera_pos.z - CAMERA_FAR) / SQRT_2 / SCALE_RATIO,
    ]
}
