use std::collections::{HashMap, HashSet};
use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub use actor::{ActorAction, ActorDirection, ActorGunHand, ResActor, ResGun};
pub use cache::{ActorAssets, Cache};
use world_generator::LevelModel;

use crate::character::CopActor;
use crate::res::cache::ActorCache;
use crate::utils;

mod actor;
mod cache;

pub const SCALE_RATIO: f32 = 0.05;
pub const GRID_SIZE: f32 = 16.0;
pub const GRID_SIZE_HALF: f32 = 8.0;

pub fn initial_res(mut cache: ResMut<Cache>, mut images: ResMut<Assets<Image>>) {
    for image in utils::split_images_to_vec("assets/art/ui/dragon.png", Vec2::new(74.0, 77.0), 8, 1)
    {
        cache.ui_title_dragon.push(images.add(image));
    }
}

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
    actor.set_tilemap_pos({
        // 把birth point设定成[1,1]了
        // [0,0]点是左下角为原点的位置
        // 但是实际放入bevy坐标系时，是不处理gird point到world point的处理的
        // 也就是说实际上x轴和y轴都偏移的8像素
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

    // 加载actor convict相关资源
    {
        let mut convict_images = ActorAssets::default();
        let mut convict_materials = ActorAssets::default();
        for (tag, frames) in utils::load_actor_sprite(
            "assets/art/character/The Convict.json",
            "assets/art/character/The Convict.png",
        ) {
            for frame in frames {
                let image_handle = images.add(frame);
                let material_handle = materials.add(StandardMaterial {
                    base_color_texture: Some(image_handle.clone()),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    depth_bias: 10.0,
                    ..default()
                });
                convict_images.insert_frame(&tag, image_handle);
                convict_materials.insert_frame(&tag, material_handle);
            }
        }
        cache
            .actors_images
            .insert(String::from("Convict"), convict_images);
        cache
            .actors_materials
            .insert(String::from("Convict"), convict_materials);
    }

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

    cache.char_hand_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(4.0 * SCALE_RATIO, 4.0 * SCALE_RATIO),
        flip: false,
    }));
    cache.char_hand_image = server.load("art/character/hand.png");
    cache.char_hand_material = materials.add(StandardMaterial {
        base_color_texture: Some(cache.char_hand_image.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        depth_bias: 5.0,
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
            0.0,
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
                size: Vec2::new(16.0 * SCALE_RATIO, 16.0 * SCALE_RATIO),
                flip: false,
            })),
        );
        cache.gun_meshes_flip.insert(
            (16, 16),
            meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(16.0 * SCALE_RATIO, 16.0 * SCALE_RATIO),
                flip: true,
            })),
        );
    }

    // 加载bullet相关贴图资源
    {
        let bullet_image = server.load("art/gun/Budget Revolver Bullet.png");
        cache
            .bullet_images
            .insert(String::from("Budget Revolver"), bullet_image.clone());
        cache.bullet_materials.insert(
            String::from("Budget Revolver"),
            materials.add(StandardMaterial {
                base_color_texture: Some(bullet_image.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                depth_bias: 15.0,
                ..default()
            }),
        );
        cache.bullet_meshes.insert(
            (5, 5),
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                5.0 * SCALE_RATIO,
                5.0 * SCALE_RATIO,
            )))),
        );
    }

    {
        for (index, image) in
            utils::split_images("assets/art/ui/heart.png", Vec2::new(15.0, 13.0), 2, 2)
        {
            cache.ui_hp_images.insert(index, images.add(image));
        }
        cache.ui_blank_image = server.load("art/ui/blank.png");
        cache.ui_key_image = server.load("art/ui/key.png");
        cache.ui_money_image = server.load("art/ui/money.png");
        cache
            .ui_card_image
            .insert(1, server.load("art/ui/gun_card.png"));

        {
            let mut ammo = None;
            let mut empty_ammo = None;
            for (index, image) in utils::split_images(
                "assets/art/ui/ammo/ammo_budget_revolver.png",
                Vec2::new(5.0, 3.0),
                2,
                1,
            ) {
                match index {
                    0 => ammo = Some(image),
                    1 => empty_ammo = Some(image),
                    _ => break,
                }
            }
            let ammo = ammo.unwrap();
            let empty_ammo = empty_ammo.unwrap();
            cache.ui_ammo_images.insert(
                String::from("budget_revolver"),
                (images.add(ammo), images.add(empty_ammo)),
            );
            cache.ui_ammo_border = server.load("art/ui/ammo_border.png")
        }
        {
            // 目前ascii的贴图中存的就是从第33个ascii码的空格开始到第127个（最后第二个）ascii码的～的像素
            for (index, image) in
                utils::split_images("assets/art/ui/font/ascii.png", Vec2::new(7.0, 9.0), 10, 10)
            {
                cache
                    .ui_ascii_font
                    .set((index + 32) as char, images.add(image));
            }
        }
    }

    {
        cache.tile_debug_mesh = meshes.add(Mesh::from(shape::UVSphere {
            radius: GRID_SIZE_HALF * SCALE_RATIO,
            ..default()
        }));
        cache.light_debug_mesh = meshes.add(Mesh::from(shape::UVSphere {
            radius: 3.0 * SCALE_RATIO,
            ..default()
        }));
        cache.tile_world_debug_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            depth_bias: 20.0,
            ..default()
        });
        cache.tile_room_debug_material = materials.add(StandardMaterial {
            base_color: Color::RED,
            unlit: true,
            depth_bias: 20.0,
            ..default()
        });
        cache.light_debug_material = materials.add(StandardMaterial {
            base_color: Color::ORANGE,
            unlit: true,
            depth_bias: 20.0,
            ..default()
        });
    }
    {
        let mut cloud_puff_materials = vec![];
        for image in utils::split_images_to_vec(
            "assets/art/actor/the_convict/cloud_puff.png",
            Vec2::new(11.0, 11.0),
            6,
            1,
        ) {
            cloud_puff_materials.push(materials.add(StandardMaterial {
                base_color_texture: Some(images.add(image)),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                depth_bias: 15.0,
                ..default()
            }));
        }
        cache.actor_caches = ActorCache {
            cloud_puff_mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                11.0 * SCALE_RATIO,
                11.0 * SCALE_RATIO,
            )))),
            cloud_puff_materials,
        };
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
        t.translation = utils::tilemap_pos_to_camera_pos(actor.get_tilemap_pos());
    }
}
