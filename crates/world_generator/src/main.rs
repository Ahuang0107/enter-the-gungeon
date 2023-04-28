use std::collections::HashMap;

use uuid::Uuid;

use world_generator::{LevelModel, Rect, RoomModel, Tile, TileGroup};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let buffer = std::fs::File::create("assets/levels/demo_output.json")?;
    // serde_json::to_writer(buffer, &world_generator::demo_level_model())?;
    let project = serde_json::from_str::<ldtk::Project>(
        std::fs::read_to_string("assets/level.ldtk")?.as_str(),
    )?;

    let level = {
        // 将ldtk中的tileset定义提取转化成需要的结构
        let (tilesets, tilesets_ref) = {
            let mut tilesets = vec![];
            let mut tilesets_ref = HashMap::new();
            for tileset_def in project.defs.tilesets.iter() {
                let size = [
                    tileset_def.tile_grid_size as u32,
                    tileset_def.tile_grid_size as u32,
                ];
                let mut tiles = HashMap::new();
                let mut count = 0;
                for y in 0..tileset_def.c_hei {
                    for x in 0..tileset_def.c_wid {
                        let min = [
                            (x as f32) * (tileset_def.tile_grid_size as f32),
                            (y as f32) * (tileset_def.tile_grid_size as f32),
                        ];
                        tiles.insert(count, Rect { min, size });
                        count += 1;
                    }
                }
                let uuid = Uuid::new_v4().to_string();
                tilesets_ref.insert(tileset_def.uid, uuid.clone());
                tilesets.push(world_generator::Tileset {
                    uuid: uuid.clone(),
                    src: tileset_def.rel_path.clone(),
                    tiles,
                    ..Default::default()
                });
            }
            (tilesets, tilesets_ref)
        };
        // 处理得到layer直接关联的tileset的uuid的关系
        let layer_to_uuid = {
            project
                .defs
                .layers
                .iter()
                .filter_map(|layer| {
                    let mut result = None;
                    if let Some(tileset_id) = layer.tileset_def_uid {
                        if let Some(tileset) = tilesets_ref.get(&tileset_id) {
                            result = Some((layer.uid, tileset.clone()))
                        }
                    }
                    result
                })
                .collect::<HashMap<usize, String>>()
        };
        // 转换所有的room
        let rooms = {
            let mut rooms = vec![];
            for level in project.levels.iter() {
                let mut room = RoomModel {
                    world_pos: [level.world_x as f32, (-level.world_y) as f32],
                    ..Default::default()
                };
                for layer in level.layer_instances.iter() {
                    if let Some(uuid) = layer_to_uuid.get(&layer.layer_def_uid) {
                        let mut tile_group = TileGroup {
                            tileset_uuid: uuid.clone(),
                            ..Default::default()
                        };
                        let used_tileset = tilesets
                            .iter()
                            .find(|tileset| tileset.uuid == *uuid)
                            .unwrap();
                        for tile in layer.grid_tiles.iter() {
                            let (index, rect) = used_tileset
                                .tiles
                                .iter()
                                .find(|(_, rect)| {
                                    rect.min[0] == tile.src.0 as f32
                                        && rect.min[1] == tile.src.1 as f32
                                })
                                .unwrap();
                            let width = rect.size[0];
                            let height = rect.size[1];
                            let x = tile.px.0 as f32 + (width / 2) as f32;
                            let y = -(tile.px.1 as f32) - (height / 2) as f32;
                            tile_group.tiles.push(Tile {
                                pos: [x, y],
                                index: *index,
                            })
                        }
                        match layer.identifier.as_str() {
                            "Roof_Stone" | "Roof_Wood" => {
                                room.roofs.push(tile_group);
                            }
                            "Carpet_Blue" | "Carpet_Red" | "Floor_Brick" | "Initial_Floor" => {
                                room.floors.push(tile_group);
                            }
                            "Wall" => room.walls.push(tile_group),
                            _ => {}
                        }
                    }
                }
                rooms.push(room);
            }
            rooms
        };
        LevelModel { rooms, tilesets }
    };
    let buffer = std::fs::File::create("assets/levels/demo_output.json")?;
    serde_json::to_writer(buffer, &level)?;
    Ok(())
}
