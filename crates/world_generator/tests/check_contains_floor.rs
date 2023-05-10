#[test]
fn check_contains_floor() {
    let level = world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
    assert!(level.contains_floor([0, 0]));
}
