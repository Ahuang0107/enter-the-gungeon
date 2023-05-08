use std::collections::HashMap;

use uuid::Uuid;

use ldtk::FieldValue;
use world_generator::{LevelModel, Light, RoomModel, TileGroup};

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
                let (size, y_range) = match tileset_def.identifier.as_str() {
                    "Wall" => {
                        tileset.tilt = true;
                        (
                            [tileset_def.tile_grid_size, (tileset_def.tile_grid_size * 2)],
                            (0..tileset_def.c_hei).step_by(2),
                        )
                    }
                    _ => (
                        [tileset_def.tile_grid_size, tileset_def.tile_grid_size],
                        (0..tileset_def.c_hei).step_by(1),
                    ),
                };
                for y in y_range {
                    for x in 0..tileset_def.c_wid {
                        let min = [
                            x * tileset_def.tile_grid_size,
                            y * tileset_def.tile_grid_size,
                        ];
                        tileset.tiles.insert(count, (min, size));
                        count += 1;
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
                let size = [level.px_wid, level.px_hei];
                let grid_size = [size[0] / 16, size[1] / 16];
                let grid_offset = [level.world_x / 16, -level.world_y / 16];
                let grid_offset = [grid_offset[0], grid_offset[1] - grid_size[1] as i32];
                let mut room = RoomModel {
                    world_pos: grid_offset,
                    size: grid_size,
                    ..Default::default()
                };
                for layer in level.layer_instances.iter() {
                    match layer.identifier.as_str() {
                        "Light" => {
                            for entity in layer.entity_instances.iter() {
                                let x = entity.px.0;
                                let y = size[1] - entity.px.1;
                                let grid_x = x / 16;
                                let grid_y = y / 16;
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
                                    pos: [grid_x, grid_y, if inner { 32 } else { 0 }],
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
                                    if let Some((index, _)) =
                                        used_tileset.tiles.iter().find(|(_, rect)| {
                                            rect.0[0] == tile.src[0] && rect.0[1] == tile.src[1]
                                        })
                                    {
                                        let grid_x = tile.px[0] / 16;
                                        let grid_y = tile.px[1] / 16;
                                        let grid_y = grid_size[1] - grid_y;
                                        tile_group.insert(grid_x, grid_y, *index);
                                    }
                                }
                                match layer.identifier.as_str() {
                                    "Roof" => {
                                        if !tile_group.tiles.is_empty() {
                                            room.roofs.push(tile_group);
                                        }
                                    }
                                    "Floor" => {
                                        if !tile_group.tiles.is_empty() {
                                            room.floors.push(tile_group);
                                        }
                                    }
                                    "Wall" => {
                                        if !tile_group.tiles.is_empty() {
                                            room.walls.push(tile_group);
                                        }
                                    }
                                    _ => {}
                                }
                            }
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
