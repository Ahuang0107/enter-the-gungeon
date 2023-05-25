use std::collections::HashMap;

use bevy::asset::Asset;
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
    // 每个actor分一个group，每组frame再分group
    pub actors_images: HashMap<String, ActorAssets<Image>>,
    pub actors_materials: HashMap<String, ActorAssets<StandardMaterial>>,
    // char的hand相关的material和mesh
    pub char_hand_image: Handle<Image>,
    pub char_hand_material: Handle<StandardMaterial>,
    pub char_hand_mesh: Handle<Mesh>,
    // gun相关的material和mesh
    pub gun_images: HashMap<String, HashMap<u8, Handle<Image>>>,
    pub gun_materials: HashMap<String, HashMap<u8, Handle<StandardMaterial>>>,
    pub gun_meshes: HashMap<(u32, u32), Handle<Mesh>>,
    pub gun_meshes_flip: HashMap<(u32, u32), Handle<Mesh>>,
    // bullet相关的material和mesh
    pub bullet_images: HashMap<String, Handle<Image>>,
    pub bullet_materials: HashMap<String, Handle<StandardMaterial>>,
    pub bullet_meshes: HashMap<(u32, u32), Handle<Mesh>>,
    // TODO need to des
    pub old_meshes: HashMap<String, Handle<Mesh>>,
    // ui相关的image
    pub ui_hp_images: HashMap<u8, Handle<Image>>,
    pub ui_blank_image: Handle<Image>,
    pub ui_key_image: Handle<Image>,
    pub ui_money_image: Handle<Image>,
    pub ui_card_image: HashMap<u8, Handle<Image>>,
    pub ui_ammo_images: HashMap<String, (Handle<Image>, Handle<Image>)>,
    pub ui_ammo_border: Handle<Image>,
    pub ui_ascii_font: AsciiFontTable,
    pub light_debug_mesh: Handle<Mesh>,
    pub light_debug_material: Handle<StandardMaterial>,
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
    pub fn get_actor_materials(&self, name: &str, tag: &str) -> &Vec<Handle<StandardMaterial>> {
        self.actors_materials.get(name).unwrap().get_frames(tag)
    }
    pub fn get_character_mesh(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile28").unwrap()
    }
    pub fn get_character_mesh_flip(&self) -> &Handle<Mesh> {
        self.old_meshes.get("Tile28Flip").unwrap()
    }
    pub fn get_gun_mesh(&self, key: (u32, u32), flip: bool) -> &Handle<Mesh> {
        if flip {
            self.gun_meshes_flip.get(&key).unwrap()
        } else {
            self.gun_meshes.get(&key).unwrap()
        }
    }
    pub fn get_gun_image(&self, tag: &str, index: u8) -> &Handle<Image> {
        self.gun_images.get(tag).unwrap().get(&index).unwrap()
    }
    pub fn get_gun_material(&self, tag: &str, index: u8) -> &Handle<StandardMaterial> {
        self.gun_materials.get(tag).unwrap().get(&index).unwrap()
    }
    pub fn get_bullet_mesh(&self, key: (u32, u32)) -> &Handle<Mesh> {
        self.bullet_meshes.get(&key).unwrap()
    }
    pub fn get_bullet_material(&self, tag: &str) -> &Handle<StandardMaterial> {
        self.bullet_materials.get(tag).unwrap()
    }
    /// 角色hp显示，0是满的一颗心，1是半颗心，2是空的心
    pub fn get_hp_image(&self, index: u8) -> &Handle<Image> {
        self.ui_hp_images.get(&index).unwrap()
    }
    /// index表示card的叠加数量，从1开始
    pub fn get_card_image(&self, index: u8) -> &Handle<Image> {
        self.ui_card_image.get(&index).unwrap()
    }
    /// name是枪械的名称，index==0是空弹药，index==1是有弹药
    pub fn get_ui_ammo_images(&self, name: &str) -> &(Handle<Image>, Handle<Image>) {
        self.ui_ammo_images.get(name).unwrap()
    }
}

#[derive(Default)]
pub struct ActorAssets<T: Asset> {
    assets: HashMap<String, Vec<Handle<T>>>,
}

impl<T: Asset> ActorAssets<T> {
    /// 指定tag插入一帧资源，并返回该帧的索引
    pub fn insert_frame(&mut self, tag: &str, frame: Handle<T>) -> usize {
        if let Some(frames) = self.assets.get_mut(tag) {
            let len = frames.len();
            frames.push(frame);
            len
        } else {
            self.assets.insert(tag.to_string(), vec![frame]);
            0
        }
    }
    pub fn get_frames(&self, tag: &str) -> &Vec<Handle<T>> {
        self.assets.get(tag).unwrap()
    }
}

/// 英文字体表
///
/// 当语言为英文时，理论上只需要显示26个英文字母大小写+10个数字
/// 都使用等宽像素字体并以贴图的形式加载进来
#[derive(Default)]
pub struct AsciiFontTable {
    table: HashMap<char, Handle<Image>>,
}

impl AsciiFontTable {
    pub fn set(&mut self, key: char, value: Handle<Image>) {
        self.table.insert(key, value);
    }
    pub fn get(&self, key: char) -> &Handle<Image> {
        self.table.get(&key).unwrap()
    }
}
