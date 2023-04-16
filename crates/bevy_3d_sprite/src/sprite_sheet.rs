use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct SpriteSheet {
    pub texture: Handle<Image>,
    pub tile_size: Vec2,
    #[reflect(ignore)]
    pub textures: Vec<Rect>,
    #[reflect(ignore)]
    pub texture_handles: HashMap<usize, Handle<Image>>,
    pub index: Option<usize>,
    pub flip_x: bool,
}

impl SpriteSheet {
    pub fn from_grid(texture: Handle<Image>, tile_size: Vec2, columns: usize, rows: usize) -> Self {
        let mut textures = Vec::new();
        for y in 0..rows {
            for x in 0..columns {
                let cell = Vec2::new(x as f32, y as f32);
                let rect_min = tile_size * cell;
                textures.push(Rect {
                    min: rect_min,
                    max: rect_min + tile_size,
                })
            }
        }
        Self {
            texture,
            tile_size,
            textures,
            texture_handles: HashMap::new(),
            index: Some(1),
            flip_x: false,
        }
    }
}

pub fn detect_and_clip_texture(
    mut image: ResMut<Assets<Image>>,
    mut query: Query<&mut SpriteSheet>,
) {
    for mut sprite_sheet in query.iter_mut() {
        if sprite_sheet.texture_handles.is_empty() {
            match image.get(&sprite_sheet.texture) {
                Some(texture) => {
                    let size = texture.texture_descriptor.size;
                    let mut texture_image =
                        image::RgbaImage::from_raw(size.width, size.height, texture.data.clone())
                            .unwrap();
                    let mut texture_handles: HashMap<usize, Handle<Image>> = HashMap::new();
                    for (index, rect) in sprite_sheet.textures.iter().enumerate() {
                        let sub_image = image::imageops::crop(
                            &mut texture_image,
                            rect.min.x as u32,
                            rect.min.y as u32,
                            sprite_sheet.tile_size.x as u32,
                            sprite_sheet.tile_size.y as u32,
                        )
                        .to_image();
                        let sub_image_handle = image.add(Image::new(
                            Extent3d {
                                width: sub_image.width(),
                                height: sub_image.height(),
                                depth_or_array_layers: 1,
                            },
                            TextureDimension::D2,
                            sub_image.into_raw(),
                            TextureFormat::Rgba8UnormSrgb,
                        ));
                        texture_handles.insert(index, sub_image_handle);
                    }
                    sprite_sheet.texture_handles = texture_handles;
                }
                None => {}
            }
        }
    }
}
