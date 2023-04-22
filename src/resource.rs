use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashMap;

const SCALE_RATIO: f32 = 0.1;

#[derive(Resource, Default)]
pub struct ResourceCache {
    pub texture_atlases: HashMap<&'static str, HashMap<usize, Handle<Image>>>,
    pub materials: HashMap<&'static str, HashMap<usize, Handle<StandardMaterial>>>,
    pub meshes: HashMap<&'static str, Handle<Mesh>>,
}

impl ResourceCache {
    pub fn tile_16(&self) -> &Handle<Mesh> {
        self.meshes.get("Tile16").unwrap()
    }
    pub fn tile_16_deg_30(&self) -> &Handle<Mesh> {
        self.meshes.get("Tile16Deg30").unwrap()
    }
    pub fn tile_24_26_deg_30(&self) -> &Handle<Mesh> {
        self.meshes.get("Tile2426Deg30").unwrap()
    }
    pub fn tile_24_26_deg_30_flip(&self) -> &Handle<Mesh> {
        self.meshes.get("Tile2426Deg30Flip").unwrap()
    }
    pub fn get_material(&self, tag: &str, index: usize) -> &Handle<StandardMaterial> {
        self.materials.get(tag).unwrap().get(&index).unwrap()
    }
}

pub fn initial_texture_atlases(
    mut cache: ResMut<ResourceCache>,
    mut image: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_16_mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(
        16.0 * SCALE_RATIO,
    ))));
    cache.meshes.insert("Tile16", tile_16_mesh_handle);
    // 倾斜30度，所以需要显示的y是a的话，实际quad的y需要是2a/√3
    // TODO `3.0_f32.sqrt()`可以直接用常量
    let tile_16_deg_30_mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        16.0 * SCALE_RATIO,
        (32.0 * SCALE_RATIO * 2.0) / 3.0_f32.sqrt(),
    ))));
    cache
        .meshes
        .insert("Tile16Deg30", tile_16_deg_30_mesh_handle);
    let tile_24_26_deg_30_mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        24.0 * SCALE_RATIO,
        (26.0 * SCALE_RATIO * 2.0) / 3.0_f32.sqrt(),
    ))));
    cache
        .meshes
        .insert("Tile2426Deg30", tile_24_26_deg_30_mesh_handle);
    let tile_24_26_deg_30_flip_mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(
            24.0 * SCALE_RATIO,
            (26.0 * SCALE_RATIO * 2.0) / 3.0_f32.sqrt(),
        ),
        flip: true,
    }));
    cache
        .meshes
        .insert("Tile2426Deg30Flip", tile_24_26_deg_30_flip_mesh_handle);

    fn initial_texture<P>(
        path: P,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        images: &mut ResMut<Assets<Image>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        is_character: bool,
    ) -> (
        HashMap<usize, Handle<Image>>,
        HashMap<usize, Handle<StandardMaterial>>,
    )
    where
        P: AsRef<std::path::Path>,
    {
        let mut texture_atlas = HashMap::new();
        let mut material_set = HashMap::new();
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
                let sub_image_handle = images.add(sub_image);
                let material_handle = if is_character {
                    materials.add(StandardMaterial {
                        base_color_texture: Some(sub_image_handle.clone()),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        depth_bias: 10.0,
                        ..default()
                    })
                } else {
                    materials.add(StandardMaterial {
                        base_color_texture: Some(sub_image_handle.clone()),
                        perceptual_roughness: 0.8,
                        metallic: 0.5,
                        reflectance: 0.1,
                        alpha_mode: AlphaMode::Blend,
                        depth_bias: 1.0,
                        ..default()
                    })
                };
                material_set.insert((y * columns + x) as usize, material_handle);
                texture_atlas.insert((y * columns + x) as usize, sub_image_handle);
            }
        }
        (texture_atlas, material_set)
    }

    let (floor_atlas, material_set) = initial_texture(
        "assets/art/floor_brick.png",
        Vec2::splat(16.0),
        3,
        2,
        &mut image,
        &mut materials,
        false,
    );
    cache.texture_atlases.insert("Floor Brick", floor_atlas);
    cache.materials.insert("Floor Brick", material_set);

    let (atlas, material_set) = initial_texture(
        "assets/art/wall.png",
        Vec2::new(16.0, 32.0),
        12,
        1,
        &mut image,
        &mut materials,
        false,
    );
    cache.texture_atlases.insert("Wall", atlas);
    cache.materials.insert("Wall", material_set);

    let (atlas, material_set) = initial_texture(
        "assets/art/roof.png",
        Vec2::splat(16.0),
        6,
        4,
        &mut image,
        &mut materials,
        false,
    );
    cache.texture_atlases.insert("Roof", atlas);
    cache.materials.insert("Roof", material_set);

    let (atlas, material_set) = initial_texture(
        "assets/art/covict.png",
        Vec2::new(24.0, 26.0),
        13,
        12,
        &mut image,
        &mut materials,
        true,
    );
    cache.texture_atlases.insert("Covict", atlas);
    cache.materials.insert("Covict", material_set);
}
