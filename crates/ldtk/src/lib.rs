use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "jsonVersion")]
    pub json_version: String,
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: usize,
    pub defs: Definitions,
    pub levels: Vec<Level>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Definitions {
    pub layers: Vec<LayerDefinition>,
    pub tilesets: Vec<TilesetDefinition>,
    pub entities: Vec<EntityDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LayerType {
    Entities,
    Tiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerDefinition {
    pub uid: usize,
    pub identifier: String,
    #[serde(rename = "__type")]
    pub type_: LayerType,
    #[serde(rename = "gridSize")]
    pub grid_size: usize,
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TilesetDefinition {
    pub uid: usize,
    pub identifier: String,
    #[serde(rename = "__cWid")]
    pub c_wid: u32,
    #[serde(rename = "__cHei")]
    pub c_hei: u32,
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: u32,
    #[serde(rename = "relPath")]
    pub rel_path: String,
    #[serde(rename = "savedSelections")]
    pub saved_selections: Vec<SavedSelection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavedSelection {
    pub ids: Vec<u8>,
    pub mode: String,
}

// impl TilesetDefinition {
//     /// 根据 grid tiles 中的 src 得到在 tileset 中的 index
//     /// index 从 1 开始，因为 0 被用来表示 null
//     pub fn get_index(&self, src: (usize, usize)) -> usize {
//         let (x, y) = src;
//         assert!(x <= (self.c_wid - 1) * self.tile_grid_size);
//         assert!(y <= (self.c_hei - 1) * self.tile_grid_size);
//         let x_i = x / self.tile_grid_size;
//         let y_i = y / self.tile_grid_size;
//         x_i + 1 + y_i * self.c_wid
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityDefinition {
    pub uid: usize,
    pub identifier: String,
    #[serde(rename = "fieldDefs")]
    pub field_defs: Vec<FieldDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDefinition {
    pub identifier: String,
    #[serde(rename = "__type")]
    type_: FieldType,
}

// #[cfg(test)]
// mod test {
//     use crate::TilesetDefinition;
//
//     #[test]
//     fn check() {
//         let tileset = TilesetDefinition {
//             uid: 0,
//             identifier: String::new(),
//             c_wid: 10,
//             c_hei: 2,
//             tile_grid_size: 16,
//             rel_path: String::new(),
//             saved_selections: vec![],
//         };
//         assert_eq!(tileset.get_index([0, 0]), 1);
//         assert_eq!(tileset.get_index([16, 0]), 2);
//         assert_eq!(tileset.get_index((32, 0)), 3);
//         assert_eq!(tileset.get_index((32, 0)), 3);
//         assert_eq!(tileset.get_index((144, 0)), 10);
//         assert_eq!(tileset.get_index((0, 16)), 11);
//         assert_eq!(tileset.get_index((16, 16)), 12);
//         assert_eq!(tileset.get_index((144, 16)), 20);
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub identifier: String,
    #[serde(rename = "pxWid")]
    pub px_wid: u32,
    #[serde(rename = "pxHei")]
    pub px_hei: u32,
    #[serde(rename = "worldX")]
    pub world_x: i32,
    #[serde(rename = "worldY")]
    pub world_y: i32,
    #[serde(rename = "layerInstances")]
    pub layer_instances: Vec<LayerInstance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__cWid")]
    pub c_wid: usize,
    #[serde(rename = "__cHei")]
    pub c_hei: usize,
    #[serde(rename = "__gridSize")]
    pub grid_size: usize,
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: usize,
    #[serde(rename = "gridTiles")]
    pub grid_tiles: Vec<GridTile>,
    #[serde(rename = "entityInstances")]
    pub entity_instances: Vec<EntityInstance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridTile {
    /// 表示在layer中的位置，根据tile的左上角定位
    pub px: [u32; 2],
    /// 表示在tileset中的位置，根据tile的左上角定位
    pub src: [u32; 2],
}

// TODO don know how to deserialize dynamic struct
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "defUid")]
    pub def_uid: usize,
    pub px: (u32, u32),
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstances>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldInstances {
    #[serde(rename = "__identifier")]
    pub identifier: String,
    #[serde(rename = "__type")]
    type_: FieldType,
    #[serde(rename = "__value")]
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldType {
    Color,
    Int,
    Float,
    Bool,
}

impl FieldInstances {
    pub fn get_value(&self) -> FieldValue {
        fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
            if hex.len() != 6 {
                return None;
            }
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        match self.type_ {
            FieldType::Color => {
                // self.value should be #FFFFFF
                let hex = self
                    .value
                    .as_str()
                    .unwrap_or_default()
                    .chars()
                    .skip(1)
                    .collect::<String>();
                if let Some((r, g, b)) = hex_to_rgb(&hex) {
                    FieldValue::Color { r, g, b }
                } else {
                    FieldValue::Color { r: 0, g: 0, b: 0 }
                }
            }
            FieldType::Int => FieldValue::Int(self.value.as_u64().unwrap_or_default() as usize),
            FieldType::Float => FieldValue::Float(self.value.as_f64().unwrap_or_default() as f32),
            FieldType::Bool => FieldValue::Bool(self.value.as_bool().unwrap_or_default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldValue {
    Color { r: u8, g: u8, b: u8 },
    Int(usize),
    Float(f32),
    Bool(bool),
}
