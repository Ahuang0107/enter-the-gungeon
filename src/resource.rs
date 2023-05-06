use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use world_generator::LevelModel;

pub const SCALE_RATIO: f32 = 0.1;

#[derive(Resource, Default)]
pub struct ResourceCache {
    pub levels: Vec<LevelModel>,
    pub images: HashMap<String, HashMap<u8, Handle<Image>>>,
    pub tile_materials: HashMap<String, HashMap<u8, Handle<StandardMaterial>>>,
    // TODO need to des
    pub old_meshes: HashMap<String, Handle<Mesh>>,
    // 主要是floor和roof使用mesh
    pub tile_plane_meshes: HashMap<(u32, u32), Handle<Mesh>>,
    // 主要是wall使用mesh
    pub tile_tilt_meshes: HashMap<(u32, u32), Handle<Mesh>>,
}

impl ResourceCache {
    pub fn get_plane_mesh(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.tile_plane_meshes.get(&key).unwrap()
    }
    pub fn get_tilt_mesh(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.tile_plane_meshes.get(&key).unwrap()
    }
    pub fn tile_24_26_deg_30(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile2426Deg30").unwrap()
    }
    pub fn tile_24_26_deg_30_flip(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile2426Deg30Flip").unwrap()
    }
    pub fn get_material(&self, tag: &str, index: u8) -> &Handle<StandardMaterial> {
        self.tile_materials.get(tag).unwrap().get(&index).unwrap()
    }
}

pub fn initial_texture_atlases(
    mut cache: ResMut<ResourceCache>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level = LevelModel::from("assets/levels/demo_output.json").unwrap();
    cache.levels.push(level.clone());

    // 收集所有的尺寸用来创建mesh
    let mut plane_meshes_set = HashSet::new();
    let mut tilt_meshes_set = HashSet::new();
    // 加载所有的 tile image
    for tileset in level.tilesets.iter() {
        let mut tileset_images = HashMap::new();
        let mut tileset_materials = HashMap::new();
        let mut dynamic_image = image::open(format!("assets/{}", tileset.src)).unwrap();
        let buffer = dynamic_image.as_mut_rgba8().unwrap();
        for (index, rect) in tileset.tiles.iter() {
            if tileset.tilt {
                tilt_meshes_set.insert((rect.size[0], rect.size[1]));
            } else {
                plane_meshes_set.insert((rect.size[0], rect.size[1]));
            }
            let sub_buffer = image::imageops::crop(
                buffer,
                rect.min[0] as u32,
                rect.min[1] as u32,
                rect.size[0] as u32,
                rect.size[1] as u32,
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
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(sub_image_handle.clone()),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                reflectance: 0.1,
                alpha_mode: AlphaMode::Blend,
                depth_bias: 1.0,
                ..default()
            });
            tileset_images.insert(*index, sub_image_handle);
            tileset_materials.insert(*index, material_handle);
        }
        cache.images.insert(tileset.uuid.clone(), tileset_images);
        cache
            .tile_materials
            .insert(tileset.uuid.clone(), tileset_materials);
    }

    // 汇总好的水平的tile的每种尺寸都创建mesh
    for (width, height) in plane_meshes_set {
        cache.tile_plane_meshes.insert(
            (width as u32, height as u32),
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                SCALE_RATIO * width as f32,
                SCALE_RATIO * height as f32,
            )))),
        );
    }
    // 汇总好的倾斜的tile的每种尺寸都创建mesh
    for (width, height) in tilt_meshes_set {
        cache.tile_plane_meshes.insert(
            (width as u32, height as u32),
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                SCALE_RATIO * width as f32,
                (SCALE_RATIO * height as f32 * 2.0) / 3.0_f32.sqrt(),
            )))),
        );
    }

    let tile_24_26_deg_30_mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        24.0 * SCALE_RATIO,
        (26.0 * SCALE_RATIO * 2.0) / 3.0_f32.sqrt(),
    ))));
    cache
        .old_meshes
        .insert(String::from("Tile2426Deg30"), tile_24_26_deg_30_mesh_handle);
    let tile_24_26_deg_30_flip_mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(
            24.0 * SCALE_RATIO,
            (26.0 * SCALE_RATIO * 2.0) / 3.0_f32.sqrt(),
        ),
        flip: true,
    }));
    cache.old_meshes.insert(
        String::from("Tile2426Deg30Flip"),
        tile_24_26_deg_30_flip_mesh_handle,
    );

    fn initial_texture<P>(
        path: P,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        images: &mut ResMut<Assets<Image>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        is_character: bool,
    ) -> (
        HashMap<u8, Handle<Image>>,
        HashMap<u8, Handle<StandardMaterial>>,
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
                        perceptual_roughness: 0.9,
                        metallic: 0.0,
                        reflectance: 0.1,
                        alpha_mode: AlphaMode::Blend,
                        depth_bias: 1.0,
                        ..default()
                    })
                };
                material_set.insert((y * columns + x) as u8, material_handle);
                texture_atlas.insert((y * columns + x) as u8, sub_image_handle);
            }
        }
        (texture_atlas, material_set)
    }

    let (atlas, material_set) = initial_texture(
        "assets/art/covict.png",
        Vec2::new(24.0, 26.0),
        13,
        12,
        &mut images,
        &mut materials,
        true,
    );
    cache.images.insert(String::from("Covict"), atlas);
    cache
        .tile_materials
        .insert(String::from("Covict"), material_set);
}
