use std::collections::HashMap;

use crate::model::Light;

use super::LevelModel;
use super::Rect;
use super::RoomModel;
use super::Tile;
use super::TileGroup;
use super::Tileset;

pub fn initial_floor_tileset() -> Tileset {
    Tileset {
        uuid: String::from("f66b3043-45ff-4f9d-8c13-af5af8a43fca"),
        src: String::from("art/floor/initial_floor.png"),
        tiles: HashMap::from([(
            1,
            Rect {
                min: [0.0, 0.0],
                size: [320, 320],
            },
        )]),
        ..Default::default()
    }
}

pub fn floor_brick_tileset() -> Tileset {
    Tileset {
        uuid: String::from("acbbf28b-7172-4233-a4a5-6d852fcd35b2"),
        src: String::from("art/floor/floor_brick.png"),
        tiles: HashMap::from([
            (1, Rect::tile(0.0, 0.0)),
            (2, Rect::tile(0.0, 16.0)),
            (
                3,
                Rect {
                    min: [16.0, 0.0],
                    size: [32, 32],
                },
            ),
            (4, Rect::tile(48.0, 0.0)),
            (5, Rect::tile(48.0, 16.0)),
        ]),
        ..Default::default()
    }
}

pub fn carpet_blue_tileset() -> Tileset {
    Tileset {
        uuid: String::from("5a799ebc-2cb7-4b08-b389-96c268803201"),
        src: String::from("art/floor/carpet_blue.png"),
        tiles: HashMap::from([
            // 9x9
            (1, Rect::tile(0.0, 0.0)),
            (2, Rect::tile(16.0, 0.0)),
            (3, Rect::tile(32.0, 0.0)),
            (4, Rect::tile(0.0, 16.0)),
            (5, Rect::tile(16.0, 16.0)),
            (6, Rect::tile(32.0, 16.0)),
            (7, Rect::tile(0.0, 32.0)),
            (8, Rect::tile(16.0, 32.0)),
            (9, Rect::tile(32.0, 32.0)),
            // 4x4
            (10, Rect::tile(48.0, 0.0)),
            (11, Rect::tile(64.0, 0.0)),
            (12, Rect::tile(48.0, 16.0)),
            (13, Rect::tile(64.0, 16.0)),
            // others
            (14, Rect::tile(48.0, 32.0)),
            (15, Rect::tile(64.0, 32.0)),
            (16, Rect::tile(80.0, 32.0)),
            (17, Rect::tile(80.0, 16.0)),
        ]),
        ..Default::default()
    }
}

pub fn carpet_red_tileset() -> Tileset {
    Tileset {
        uuid: String::from("b495957d-2eaf-4409-b159-4c7a4c143b78"),
        src: String::from("art/floor/carpet_red.png"),
        tiles: HashMap::from([
            // 9x9
            (1, Rect::tile(0.0, 0.0)),
            (2, Rect::tile(16.0, 0.0)),
            (3, Rect::tile(32.0, 0.0)),
            (4, Rect::tile(0.0, 16.0)),
            (5, Rect::tile(16.0, 16.0)),
            (6, Rect::tile(32.0, 16.0)),
            (7, Rect::tile(0.0, 32.0)),
            (8, Rect::tile(16.0, 32.0)),
            (9, Rect::tile(32.0, 32.0)),
            // 4x4
            (10, Rect::tile(48.0, 0.0)),
            (11, Rect::tile(64.0, 0.0)),
            (12, Rect::tile(48.0, 16.0)),
            (13, Rect::tile(64.0, 16.0)),
            // others
            (14, Rect::tile(48.0, 32.0)),
            (15, Rect::tile(64.0, 32.0)),
        ]),
        ..Default::default()
    }
}

pub fn wall_tileset() -> Tileset {
    Tileset {
        uuid: String::from("64218b09-1cf3-4005-9022-5fa9ff6560d8"),
        src: String::from("art/wall/wall.png"),
        tiles: HashMap::from([
            (1, Rect::wall_tile(0.0, 0.0)),
            (2, Rect::wall_tile(16.0, 0.0)),
            (3, Rect::wall_tile(32.0, 0.0)),
            (4, Rect::wall_tile(48.0, 0.0)),
            (5, Rect::wall_tile(64.0, 0.0)),
            (6, Rect::wall_tile(80.0, 0.0)),
            (7, Rect::wall_tile(96.0, 0.0)),
            (8, Rect::wall_tile(112.0, 0.0)),
            (9, Rect::wall_tile(128.0, 0.0)),
            (10, Rect::wall_tile(144.0, 0.0)),
            (11, Rect::wall_tile(160.0, 0.0)),
            (12, Rect::wall_tile(176.0, 0.0)),
        ]),
        tilt: true,
    }
}

pub fn roof_stone_tileset() -> Tileset {
    Tileset {
        uuid: String::from("b1538f24-835e-4dc4-96c7-cf59b2c5d531"),
        src: String::from("art/roof/roof-stone.png"),
        tiles: HashMap::from([
            // 9x9
            (1, Rect::tile(0.0, 0.0)),
            (2, Rect::tile(16.0, 0.0)),
            (3, Rect::tile(32.0, 0.0)),
            (4, Rect::tile(0.0, 16.0)),
            (5, Rect::tile(16.0, 16.0)),
            (6, Rect::tile(32.0, 16.0)),
            (7, Rect::tile(0.0, 32.0)),
            (8, Rect::tile(16.0, 32.0)),
            (9, Rect::tile(32.0, 32.0)),
            // 1x3
            (10, Rect::tile(48.0, 0.0)),
            (11, Rect::tile(48.0, 16.0)),
            (12, Rect::tile(48.0, 32.0)),
            // 3x1
            (13, Rect::tile(0.0, 48.0)),
            (14, Rect::tile(16.0, 48.0)),
            (15, Rect::tile(32.0, 48.0)),
            // 1x1
            (16, Rect::tile(48.0, 48.0)),
            // 4x4
            (17, Rect::tile(64.0, 0.0)),
            (18, Rect::tile(80.0, 0.0)),
            (19, Rect::tile(64.0, 16.0)),
            (20, Rect::tile(80.0, 16.0)),
        ]),
        ..Default::default()
    }
}

pub fn roof_wood_tileset() -> Tileset {
    Tileset {
        uuid: String::from("a3768f3b-7fc9-4c3d-abe3-201a313c9a64"),
        src: String::from("art/roof/roof-wood.png"),
        tiles: HashMap::from([
            // 9x9
            (1, Rect::tile(0.0, 0.0)),
            (2, Rect::tile(16.0, 0.0)),
            (3, Rect::tile(32.0, 0.0)),
            (4, Rect::tile(0.0, 16.0)),
            (5, Rect::tile(16.0, 16.0)),
            (6, Rect::tile(32.0, 16.0)),
            (7, Rect::tile(0.0, 32.0)),
            (8, Rect::tile(16.0, 32.0)),
            (9, Rect::tile(32.0, 32.0)),
            // 1x3
            (10, Rect::tile(48.0, 0.0)),
            (11, Rect::tile(48.0, 16.0)),
            (12, Rect::tile(48.0, 32.0)),
            // 3x1
            (13, Rect::tile(0.0, 48.0)),
            (14, Rect::tile(16.0, 48.0)),
            (15, Rect::tile(32.0, 48.0)),
            // 1x1
            (16, Rect::tile(48.0, 48.0)),
            // 4x4
            (17, Rect::tile(64.0, 0.0)),
            (18, Rect::tile(80.0, 0.0)),
            (19, Rect::tile(64.0, 16.0)),
            (20, Rect::tile(80.0, 16.0)),
        ]),
        ..Default::default()
    }
}

impl Rect {
    fn tile(x: f32, y: f32) -> Self {
        Self {
            min: [x, y],
            size: [16, 16],
        }
    }
    fn wall_tile(x: f32, y: f32) -> Self {
        Self {
            min: [x, y],
            size: [16, 32],
        }
    }
}

fn tilesets() -> Vec<Tileset> {
    vec![
        initial_floor_tileset(),
        floor_brick_tileset(),
        carpet_blue_tileset(),
        carpet_red_tileset(),
        wall_tileset(),
        roof_stone_tileset(),
        roof_wood_tileset(),
    ]
}

fn initial_room() -> RoomModel {
    let mut wall_tiles = vec![];
    for x in 0..20 {
        let x = 8.0 + (x as f32) * 16.0;
        wall_tiles.push(Tile {
            pos: [x, 16.0],
            index: 1,
        });
    }
    wall_tiles.push(Tile {
        pos: [328.0, -208.0],
        index: 1,
    });
    wall_tiles.push(Tile {
        pos: [344.0, -208.0],
        index: 1,
    });
    let mut roof_tiles = vec![];
    for i in 0..20 {
        let x = 8.0 + (i as f32) * 16.0;
        roof_tiles.push(Tile {
            pos: [x, 40.0],
            index: 8,
        });
        roof_tiles.push(Tile {
            pos: [x, -312.0],
            index: 2,
        });
    }
    for i in 0..21 {
        let y = 24.0 - (i as f32) * 16.0;
        roof_tiles.push(Tile {
            pos: [-8.0, y],
            index: 6,
        });
        if [13, 14, 15, 16, 17].contains(&i) {
            continue;
        }
        roof_tiles.push(Tile {
            pos: [328.0, y],
            index: 4,
        });
    }
    roof_tiles.push(Tile {
        pos: [328.0, -184.0],
        index: 7,
    });
    roof_tiles.push(Tile {
        pos: [344.0, -184.0],
        index: 8,
    });
    roof_tiles.push(Tile {
        pos: [328.0, -248.0],
        index: 1,
    });
    roof_tiles.push(Tile {
        pos: [344.0, -248.0],
        index: 2,
    });
    roof_tiles.push(Tile {
        pos: [-8.0, 40.0],
        index: 20,
    });
    roof_tiles.push(Tile {
        pos: [328.0, 40.0],
        index: 19,
    });
    roof_tiles.push(Tile {
        pos: [-8.0, -312.0],
        index: 18,
    });
    roof_tiles.push(Tile {
        pos: [328.0, -312.0],
        index: 17,
    });
    RoomModel {
        world_pos: [0.0, 0.0],
        walls: vec![TileGroup {
            tileset_uuid: String::from("64218b09-1cf3-4005-9022-5fa9ff6560d8"),
            tiles: wall_tiles,
        }],
        floors: vec![
            TileGroup {
                tileset_uuid: String::from("f66b3043-45ff-4f9d-8c13-af5af8a43fca"),
                tiles: vec![Tile {
                    pos: [160.0, -160.0],
                    index: 1,
                }],
            },
            TileGroup {
                tileset_uuid: String::from("acbbf28b-7172-4233-a4a5-6d852fcd35b2"),
                tiles: vec![
                    Tile {
                        pos: [328.0, -232.0],
                        index: 1,
                    },
                    Tile {
                        pos: [344.0, -232.0],
                        index: 1,
                    },
                ],
            },
        ],
        roofs: vec![TileGroup {
            tileset_uuid: String::from("b1538f24-835e-4dc4-96c7-cf59b2c5d531"),
            tiles: roof_tiles,
        }],
        lights: vec![Light {
            pos: [8.0, 16.0],
            color: [255, 255, 255, 255],
        }],
    }
}

fn test_room_01() -> RoomModel {
    let mut floor_tiles = vec![];
    for i in 0..14 {
        let y = -24.0 - (i as f32) * 16.0;
        floor_tiles.push(Tile {
            pos: [8.0, y],
            index: 4,
        });
    }
    for i in 0..30 {
        let x = 24.0 + (i as f32) * 16.0;
        floor_tiles.push(Tile {
            pos: [x, -8.0],
            index: 4,
        });
    }
    let mut wall_tiles = vec![];
    wall_tiles.push(Tile {
        pos: [8.0, 0.0],
        index: 12,
    });
    RoomModel {
        world_pos: [368.0, -96.0],
        floors: vec![TileGroup {
            tileset_uuid: String::from("acbbf28b-7172-4233-a4a5-6d852fcd35b2"),
            tiles: floor_tiles,
        }],
        walls: vec![TileGroup {
            tileset_uuid: String::from("64218b09-1cf3-4005-9022-5fa9ff6560d8"),
            tiles: wall_tiles,
        }],
        ..Default::default()
    }
}

pub fn demo_level_model() -> LevelModel {
    LevelModel {
        rooms: vec![initial_room(), test_room_01()],
        tilesets: tilesets(),
    }
}
