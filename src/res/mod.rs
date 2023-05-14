use std::collections::{HashMap, HashSet};
use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub use actor::{ActorAction, ActorDirection, ResActor, ResGun};
pub use cache::Cache;
use world_generator::LevelModel;

use crate::character::CopActor;
use crate::{utils, CAMERA_FAR};

mod actor;
mod cache;

pub const SCALE_RATIO: f32 = 0.1;
pub const GRID_SIZE: f32 = 16.0;
pub const GRID_SIZE_HALF: f32 = 8.0;

pub fn reset_res(
    mut cache: ResMut<Cache>,
    mut actor: ResMut<ResActor>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    let level = LevelModel::from("assets/levels/demo_output.json").unwrap();
    cache.levels.push(level.clone());
    actor.update_pos({
        let pos = level.brith_point;
        [pos[0] as f32 * GRID_SIZE, pos[1] as f32 * GRID_SIZE]
    });

    // 收集所有的尺寸用来创建mesh
    let mut tile_meshes_set = HashSet::new();
    // 加载所有的 tile image
    for tileset in level.tilesets.iter() {
        let mut tileset_images = HashMap::new();
        let mut tileset_materials = HashMap::new();
        let mut dynamic_image = image::open(format!("assets/{}", tileset.src)).unwrap();
        let buffer = dynamic_image.as_mut_rgba8().unwrap();
        for (index, rect) in tileset.tiles.iter() {
            tile_meshes_set.insert((rect.1[0], rect.1[1]));
            let sub_buffer = image::imageops::crop(
                buffer,
                rect.0[0] as u32,
                rect.0[1] as u32,
                rect.1[0] as u32,
                rect.1[1] as u32,
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
        cache
            .tile_images
            .insert(tileset.uuid.clone(), tileset_images);
        cache
            .tile_materials
            .insert(tileset.uuid.clone(), tileset_materials);
    }

    // 汇总好的tile的每种尺寸都创建mesh
    for (width, height) in tile_meshes_set {
        cache.tile_meshes.insert(
            (width as u32, height as u32),
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                SCALE_RATIO * width as f32,
                SCALE_RATIO * height as f32,
            )))),
        );
        cache.tile_meshes_sqrt2.insert(
            (width as u32, height as u32),
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                SCALE_RATIO * width as f32,
                SCALE_RATIO * height as f32 * SQRT_2,
            )))),
        );
    }

    cache.old_meshes.insert(
        String::from("Tile28"),
        meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            28.0 * SCALE_RATIO,
            28.0 * SCALE_RATIO,
        )))),
    );
    cache.old_meshes.insert(
        String::from("Tile28Flip"),
        meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(28.0 * SCALE_RATIO, 28.0 * SCALE_RATIO),
            flip: true,
        })),
    );

    fn initial_texture<P>(
        path: P,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        images: &mut ResMut<Assets<Image>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        depth_bias: f32,
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
                let material_handle = materials.add(StandardMaterial {
                    base_color_texture: Some(sub_image_handle.clone()),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    depth_bias,
                    ..default()
                });
                material_set.insert((y * columns + x) as u8, material_handle);
                texture_atlas.insert((y * columns + x) as u8, sub_image_handle);
            }
        }
        (texture_atlas, material_set)
    }

    let (atlas, material_set) = initial_texture(
        "assets/art/character/The Covict.png",
        Vec2::new(28.0, 28.0),
        9,
        9,
        &mut images,
        &mut materials,
        10.0,
    );
    cache.tile_images.insert(String::from("Covict"), atlas);
    cache
        .tile_materials
        .insert(String::from("Covict"), material_set);

    cache.char_hand_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(4.0 * SCALE_RATIO, 4.0 * SCALE_RATIO * SQRT_2),
        flip: false,
    }));
    cache.char_hand_image = server.load("art/character/hand.png");
    cache.char_hand_material = materials.add(StandardMaterial {
        base_color_texture: Some(cache.char_hand_image.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        depth_bias: 30.0,
        ..default()
    });

    // 加载gun相关贴图资源
    {
        let (atlas, material_set) = initial_texture(
            "assets/art/gun/Budget Revolver.png",
            Vec2::new(16.0, 16.0),
            4,
            2,
            &mut images,
            &mut materials,
            20.0,
        );
        cache
            .gun_images
            .insert(String::from("Budget Revolver"), atlas);
        cache
            .gun_materials
            .insert(String::from("Budget Revolver"), material_set);
        cache.gun_meshes.insert(
            (16, 16),
            meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(16.0 * SCALE_RATIO, 16.0 * SCALE_RATIO * SQRT_2),
                flip: false,
            })),
        );
        cache.gun_meshes_flip.insert(
            (16, 16),
            meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(16.0 * SCALE_RATIO, 16.0 * SCALE_RATIO * SQRT_2),
                flip: true,
            })),
        );
    }

    {
        for (index, image) in
            utils::split_images("assets/art/ui/heart.png", Vec2::new(16.0, 16.0), 2, 2)
        {
            cache.ui_hp_images.insert(index, images.add(image));
        }
    }
}

pub fn update_actor(
    actor: Res<ResActor>,
    mut actor_query: Query<&mut Transform, (With<CopActor>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CopActor>)>,
) {
    for mut t in actor_query.iter_mut() {
        t.translation = actor.get_actual_pos();
    }
    for mut t in camera_query.iter_mut() {
        let mut actual_pos = actor.get_actual_pos();
        actual_pos.z += CAMERA_FAR;
        t.translation = actual_pos;
    }
}
