use crate::layer::{EntityLayer, Light};
use crate::{TilemapRoom, TilesLayer, Tileset};
use ldtk::{FieldValue, LayerType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapWorld {
    pub rooms: Vec<TilemapRoom>,
    pub tilesets: Vec<Tileset>,
}

impl TilemapWorld {
    pub fn from_ldtk<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let project =
            serde_json::from_str::<ldtk::Project>(std::fs::read_to_string(path)?.as_str())?;

        let layer_to_tileset = {
            let tileset_map = project
                .defs
                .tilesets
                .iter()
                .map(|tileset| (tileset.uid, tileset))
                .collect::<std::collections::HashMap<usize, &ldtk::TilesetDefinition>>();
            project
                .defs
                .layers
                .iter()
                .filter_map(|layer| {
                    let mut result = None;
                    if let Some(tileset_id) = layer.tileset_def_uid {
                        if let Some(&tileset) = tileset_map.get(&tileset_id) {
                            result = Some((layer.uid, tileset))
                        }
                    }
                    result
                })
                .collect::<std::collections::HashMap<usize, &ldtk::TilesetDefinition>>()
        };
        let layer_defs = project
            .defs
            .layers
            .iter()
            .map(|layer| (layer.uid, layer))
            .collect::<std::collections::HashMap<usize, &ldtk::LayerDefinition>>();

        let mut rooms: Vec<TilemapRoom> = vec![];
        for level in project.levels.iter() {
            let columns = level.px_wid / project.default_grid_size;
            let rows = level.px_hei / project.default_grid_size;
            let mut tilemap_room = TilemapRoom::new(level.world_x as f32, level.world_y as f32);
            for layer in level.layer_instances.iter() {
                let &layer_def = layer_defs.get(&layer.layer_def_uid).unwrap();
                match layer_def.type_ {
                    LayerType::Entities => {
                        // 这里为了配合项目做一点小trick，单独提取出Light的信息
                        if layer_def.identifier == "Light" {
                            assert!(!layer.entity_instances.is_empty());

                            let mut lights = vec![];
                            for entity_instance in layer.entity_instances.iter() {
                                let mut color = None;
                                let mut alpha = None;
                                let mut height = None;
                                for field in entity_instance.field_instances.iter() {
                                    match field.get_value() {
                                        FieldValue::Color { r, g, b } => color = Some([r, g, b]),
                                        FieldValue::Int(a) => alpha = Some(a as u8),
                                        FieldValue::Float(h) => height = Some(h),
                                    }
                                }
                                let color = color.unwrap();
                                let alpha = alpha.unwrap();
                                let height = height.unwrap();
                                lights.push(Light {
                                    pos: [
                                        entity_instance.px.0 as f32,
                                        entity_instance.px.1 as f32,
                                        height,
                                    ],
                                    color: [color[0], color[1], color[2], alpha],
                                })
                            }
                            tilemap_room.push_entities_layer(EntityLayer { lights });
                        }
                    }
                    LayerType::Tiles => {
                        if let Some(&tileset) = layer_to_tileset.get(&layer.layer_def_uid) {
                            let tile_size = layer.grid_size;

                            let mut layer_vec = vec![];

                            for tile in layer.grid_tiles.iter() {
                                let (x, y) = tile.px;
                                let x_i = x / tile_size;
                                let y_i = y / tile_size;
                                layer_vec.push((x_i, y_i, tileset.get_index(tile.src)));
                            }

                            tilemap_room.push_tiles_layer(TilesLayer::new(
                                (tile_size as f32, tile_size as f32),
                                columns,
                                rows,
                                layer_vec,
                                &tileset.identifier,
                            ));
                        }
                    }
                }
            }
            rooms.push(tilemap_room);
        }
        // 收集所有贴图的集合
        let tilesets = project
            .defs
            .tilesets
            .iter()
            .map(|tileset| {
                // 这里为了配合项目需要一点小trick，墙壁的贴图并不是正方形的，需要做一个合并操作
                if tileset.identifier == "Wall" {
                    Tileset::new(
                        &tileset.identifier,
                        &tileset.rel_path,
                        (
                            tileset.tile_grid_size as f32,
                            (tileset.tile_grid_size * 2) as f32,
                        ),
                        tileset.c_wid,
                        tileset.c_hei / 2,
                    )
                } else {
                    Tileset::from(tileset)
                }
            })
            .collect::<Vec<Tileset>>();
        Ok(Self { rooms, tilesets })
    }
}
