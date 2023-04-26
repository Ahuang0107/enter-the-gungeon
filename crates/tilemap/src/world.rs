use crate::{TilemapLayer, TilemapRoom, Tileset};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TilemapWorld {
    pub levels: Vec<TilemapRoom>,
    pub tilesets: Vec<Tileset>,
}

impl TilemapWorld {
    pub fn from_ldtk<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
        let mut levels: Vec<TilemapRoom> = vec![];
        for level in project.levels.iter() {
            let columns = level.px_wid / project.default_grid_size;
            let rows = level.px_hei / project.default_grid_size;
            let mut tilemap_room = TilemapRoom::new(level.world_x as f32, level.world_y as f32);
            for layer in level.layer_instances.iter() {
                let tileset_id = layer_tileset_id_map.get(&layer.layer_def_uid).unwrap();
                let &tileset = tileset_map.get(tileset_id).unwrap();

                let tile_size = layer.grid_size;

                let mut layer_vec = vec![];

                for tile in layer.grid_tiles.iter() {
                    let (x, y) = tile.px;
                    let x_i = x / tile_size;
                    let y_i = y / tile_size;
                    layer_vec.push((x_i, y_i, tileset.get_index(tile.src)));
                }

                tilemap_room.push_layer(TilemapLayer::new(
                    (tile_size as f32, tile_size as f32),
                    columns,
                    rows,
                    layer_vec,
                    &tileset.identifier,
                ));
            }
            levels.push(tilemap_room);
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
