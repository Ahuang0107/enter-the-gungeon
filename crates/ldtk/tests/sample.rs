use ldtk::Project;

#[test]
fn check() {
    let project = serde_json::from_str::<Project>(
        std::fs::read_to_string("tests/sample.ldtk")
            .unwrap()
            .as_str(),
    )
    .unwrap();
    assert_eq!(&project.json_version, "1.2.5");
    let first_tile = &project.levels[0].layer_instances[0].grid_tiles[0];
    assert_eq!(first_tile.px, [16, 0]);
    assert_eq!(first_tile.src, [64, 0]);
}

#[test]
fn check_real() {
    serde_json::from_str::<Project>(
        std::fs::read_to_string("../../assets/level.ldtk")
            .unwrap()
            .as_str(),
    )
    .unwrap();
}
