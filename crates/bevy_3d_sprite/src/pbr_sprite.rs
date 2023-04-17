use bevy::prelude::*;
use bevy::render::render_resource::Face;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Bundle, Clone, Default)]
pub struct PbrSpriteBundle {
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprite: TextureAtlasSprite,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

pub fn update_base_texture(
    mut query: Query<(
        &Handle<TextureAtlas>,
        &TextureAtlasSprite,
        &Handle<StandardMaterial>,
        &mut Transform,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas_handle, sprite, standard_material_handle, mut transform) in query.iter_mut()
    {
        if let Some(standard_material) = materials.get_mut(standard_material_handle) {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                if let Some(index) = sprite.index {
                    if let Some(texture) = texture_atlas.texture_handles.get(&index) {
                        standard_material.base_color = Color::WHITE;
                        standard_material.base_color_texture = Some(texture.clone());
                        if let Some(flip_x) = sprite.flip_x {
                            if flip_x {
                                standard_material.cull_mode = Some(Face::Front);
                                transform.rotation = Quat::from_rotation_y(-PI);
                            } else {
                                standard_material.cull_mode = Some(Face::Back);
                                transform.rotation = Quat::IDENTITY;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, Reflect, bevy::reflect::TypeUuid)]
#[uuid = "b11407d4-d4b1-4d68-ab32-a96e5519b27b"]
pub struct TextureAtlas {
    pub texture: Handle<Image>,
    pub tile_size: Vec2,
    #[reflect(ignore)]
    pub textures: Vec<Rect>,
    #[reflect(ignore)]
    pub texture_handles: HashMap<usize, Handle<Image>>,
}

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct TextureAtlasSprite {
    pub index: Option<usize>,
    pub flip_x: Option<bool>,
}

impl TextureAtlas {
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
        }
    }
}

pub fn detect_and_clip_texture(
    mut image: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // TODO 这里应该不需要限制 With<TextureAtlasSprite>
    mut query: Query<&Handle<TextureAtlas>, With<TextureAtlasSprite>>,
) {
    for texture_atlas in query.iter_mut() {
        if let Some(texture_atlas_handle) = texture_atlases.get_mut(texture_atlas) {
            if texture_atlas_handle.texture_handles.is_empty() {
                match image.get(&texture_atlas_handle.texture) {
                    Some(texture) => {
                        let size = texture.texture_descriptor.size;
                        let mut texture_image = image::RgbaImage::from_raw(
                            size.width,
                            size.height,
                            texture.data.clone(),
                        )
                        .unwrap();
                        let mut texture_handles: HashMap<usize, Handle<Image>> = HashMap::new();
                        for (index, rect) in texture_atlas_handle.textures.iter().enumerate() {
                            let sub_image = image::imageops::crop(
                                &mut texture_image,
                                rect.min.x as u32,
                                rect.min.y as u32,
                                texture_atlas_handle.tile_size.x as u32,
                                texture_atlas_handle.tile_size.y as u32,
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
                        texture_atlas_handle.texture_handles = texture_handles;
                    }
                    None => {}
                }
            }
        }
    }
}
