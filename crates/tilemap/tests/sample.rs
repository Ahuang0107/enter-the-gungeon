use tilemap::TilemapWorld;

#[test]
fn check() {
    let world = TilemapWorld::from_ldtk("tests/sample.ldtk").unwrap();
    assert_eq!(world.levels.len(), 1);
    let level = &world.levels[0];
    assert_eq!(level.c_w, 31);
    assert_eq!(level.c_h, 29);
    assert_eq!(level.tile_size, 16);
    assert_eq!(level.layers.len(), 5);
}
