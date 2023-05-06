use std::collections::HashMap;

use uuid::Uuid;

use ldtk::FieldValue;
use world_generator::{LevelModel, Light, Rect, RoomModel, Tile, TileGroup};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = serde_json::from_str::<ldtk::Project>(
        std::fs::read_to_string("assets/level.ldtk")?.as_str(),
    )?;

    let level = {
        // 将ldtk中的tileset定义提取转化成需要的结构
        let (tilesets, tilesets_ref) = {
            let mut tilesets = vec![];
            let mut tilesets_ref = HashMap::new();
            for tileset_def in project.defs.tilesets.iter() {
                let uuid = Uuid::new_v4().to_string();
                tilesets_ref.insert(tileset_def.uid, uuid.clone());
                let mut tileset = world_generator::Tileset {
                    uuid: uuid.clone(),
                    src: tileset_def.rel_path.clone(),
                    ..Default::default()
                };
                let mut count = 0;
                // 针对wall做一下特殊处理
                if tileset_def.identifier == "Wall" {
                    let size = [
                        tileset_def.tile_grid_size as i32,
                        (tileset_def.tile_grid_size * 2) as i32,
                    ];
                    tileset.tilt = true;
                    for y in (0..tileset_def.c_hei).step_by(2) {
                        for x in 0..tileset_def.c_wid {
                            let min = [
                                (x as i32) * (tileset_def.tile_grid_size as i32),
                                (y as i32) * (tileset_def.tile_grid_size as i32),
                            ];
                            tileset.tiles.insert(count, Rect { min, size });
                            count += 1;
                        }
                    }
                } else {
                    let size = [
                        tileset_def.tile_grid_size as i32,
                        tileset_def.tile_grid_size as i32,
                    ];
                    for y in 0..tileset_def.c_hei {
                        for x in 0..tileset_def.c_wid {
                            let min = [
                                (x as i32) * (tileset_def.tile_grid_size as i32),
                                (y as i32) * (tileset_def.tile_grid_size as i32),
                            ];
                            tileset.tiles.insert(count, Rect { min, size });
                            count += 1;
                        }
                    }
                }
                tilesets.push(tileset);
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
                    world_pos: [level.world_x as i32, (-level.world_y) as i32],
                    size: [level.px_wid as i32, level.px_hei as i32],
                    ..Default::default()
                };
                let mut walkable_area = vec![];
                for layer in level.layer_instances.iter() {
                    match layer.identifier.as_str() {
                        "Light" => {
                            for entity in layer.entity_instances.iter() {
                                let x = entity.px.0 as i32;
                                let y = -(entity.px.1 as i32);
                                let mut color = None;
                                let mut alpha = None;
                                let mut inner = None;
                                for field in entity.field_instances.iter() {
                                    match field.get_value() {
                                        FieldValue::Color { r, g, b } => color = Some([r, g, b]),
                                        FieldValue::Int(a) => alpha = Some(a as u8),
                                        FieldValue::Bool(i) => inner = Some(i),
                                        _ => {}
                                    }
                                }
                                let color = color.unwrap();
                                let alpha = alpha.unwrap();
                                let inner = inner.unwrap();
                                room.lights.push(Light {
                                    // TODO 这里的问题是如何让灯光低于roof但是能够让光扩散的足够开
                                    pos: [x, y, if inner { 32 } else { 0 }],
                                    color: [color[0], color[1], color[2], alpha],
                                })
                            }
                        }
                        _ => {
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
                                    if let Some((index, rect)) =
                                        used_tileset.tiles.iter().find(|(_, rect)| {
                                            rect.min[0] == tile.src.0 as i32
                                                && rect.min[1] == tile.src.1 as i32
                                        })
                                    {
                                        let width = rect.size[0];
                                        let height = rect.size[1];
                                        let x = tile.px.0 as i32 + (width / 2) as i32;
                                        let y = -(tile.px.1 as i32) - (height / 2) as i32;
                                        tile_group.tiles.push(Tile {
                                            pos: [x, y],
                                            index: *index,
                                        });
                                        match layer.identifier.as_str() {
                                            "Carpet_Blue" | "Carpet_Red" | "Floor_Brick"
                                            | "Initial_Floor" => walkable_area.push(Rect {
                                                min: [tile.px.0 as i32, -(tile.px.1 as i32)],
                                                size: [width, height],
                                            }),
                                            _ => {}
                                        }
                                    }
                                }
                                match layer.identifier.as_str() {
                                    "Roof_Stone" | "Roof_Wood" => {
                                        room.roofs.push(tile_group);
                                    }
                                    "Carpet_Blue" | "Carpet_Red" | "Floor_Brick"
                                    | "Initial_Floor" => {
                                        room.floors.push(tile_group);
                                    }
                                    "Wall" => room.walls.push(tile_group),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                room.walkable_area = walkable_area;
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
