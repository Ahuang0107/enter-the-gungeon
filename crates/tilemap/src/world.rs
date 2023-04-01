use crate::{TilemapLayer, TilemapLevel, Tileset};

#[derive(Debug)]
pub struct TilemapWorld {
    pub levels: Vec<TilemapLevel>,
    pub tilesets: Vec<Tileset>,
}

impl TilemapWorld {
    pub fn from_ldtk(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let project =
            serde_json::from_str::<ldtk::Project>(std::fs::read_to_string(path)?.as_str())?;
        let layer_tileset_id_map = project
            .defs
            .layers
            .iter()
            .map(|layer| (layer.uid, layer.tileset_def_uid))
            .collect::<std::collections::HashMap<usize, usize>>();
        let tileset_map = project
            .defs
            .tilesets
            .iter()
            .map(|tileset| (tileset.uid, tileset))
            .collect::<std::collections::HashMap<usize, &ldtk::TilesetDefinition>>();
        let mut levels: Vec<TilemapLevel> = vec![];
        for level in project.levels.iter() {
            let mut tilemap_level = TilemapLevel::new(
                level.px_wid / project.default_grid_size,
                level.px_hei / project.default_grid_size,
                project.default_grid_size,
            );
            for layer in level.layer_instances.iter() {
                let tileset_id = layer_tileset_id_map.get(&layer.layer_def_uid).unwrap();
                let &tileset = tileset_map.get(tileset_id).unwrap();

                let tile_size = layer.grid_size;

                let mut layer_matrix = vec![vec![0_u8; layer.c_wid]; layer.c_hei];

                for tile in layer.grid_tiles.iter() {
                    let (x, y) = tile.px;
                    let x_i = x / tile_size;
                    let y_i = y / tile_size;
                    layer_matrix[y_i][x_i] = tileset.get_index(tile.src) as u8;
                }

                tilemap_level.push_layer(
                    TilemapLayer::from_vec(*tileset_id, layer_matrix).with_name(&layer.identifier),
                );
            }
            levels.push(tilemap_level);
        }
        Ok(Self {
            levels,
            tilesets: project
                .defs
                .tilesets
                .iter()
                .map(|tileset| Tileset::from(tileset))
                .collect::<Vec<Tileset>>(),
        })
    }
}
