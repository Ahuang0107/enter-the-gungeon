use std::collections::HashMap;

use bevy::prelude::*;

use world_generator::LevelModel;

#[derive(Resource, Default)]
pub struct Cache {
    pub levels: Vec<LevelModel>,
    // 下面4个都是tilemap会用到的material和mesh
    pub tile_images: HashMap<String, HashMap<u8, Handle<Image>>>,
    pub tile_materials: HashMap<String, HashMap<u8, Handle<StandardMaterial>>>,
    // 主要是tilemap使用mesh
    pub tile_meshes: HashMap<(u32, u32), Handle<Mesh>>,
    pub tile_meshes_sqrt2: HashMap<(u32, u32), Handle<Mesh>>,
    // char的hand相关的material和mesh
    pub char_hand_image: Handle<Image>,
    pub char_hand_material: Handle<StandardMaterial>,
    pub char_hand_mesh: Handle<Mesh>,
    // gun相关的material和mesh
    pub gun_images: HashMap<String, HashMap<u8, Handle<Image>>>,
    pub gun_materials: HashMap<String, HashMap<u8, Handle<StandardMaterial>>>,
    pub gun_meshes: HashMap<(u32, u32), Handle<Mesh>>,
    // TODO need to des
    pub old_meshes: HashMap<String, Handle<Mesh>>,
    pub ui_hp_images: HashMap<u8, Handle<Image>>,
}

impl Cache {
    pub fn get_tile_mesh(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.tile_meshes.get(&key).unwrap()
    }
    pub fn get_tile_mesh_sqrt2(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.tile_meshes_sqrt2.get(&key).unwrap()
    }
    pub fn get_tile_material(&self, tag: &str, index: u8) -> &Handle<StandardMaterial> {
        self.tile_materials.get(tag).unwrap().get(&index).unwrap()
    }
    pub fn get_character_mesh(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile28").unwrap()
    }
    pub fn get_character_mesh_flip(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile28Flip").unwrap()
    }
    pub fn get_gun_mesh(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.gun_meshes.get(&key).unwrap()
    }
    pub fn get_gun_material(&self, tag: &str, index: u8) -> &Handle<StandardMaterial> {
        self.gun_materials.get(tag).unwrap().get(&index).unwrap()
    }
    pub fn get_hp_image(&self, index: u8) -> &Handle<Image> {
        self.ui_hp_images.get(&index).unwrap()
    }
}
