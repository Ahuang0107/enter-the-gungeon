#[test]
fn output_tileset_split_png() {
    let level = world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
    std::fs::create_dir_all("../../assets/tests/tilesets").unwrap();
    for tileset in level.tilesets.iter() {
        let mut dynamic_image = image::open(format!("../../assets/{}", tileset.src)).unwrap();
        let buffer = dynamic_image.as_mut_rgba8().unwrap();
        for (index, rect) in tileset.tiles.iter() {
            let sub_buffer = image::imageops::crop(
                buffer,
                rect.min[0] as u32,
                rect.min[1] as u32,
                rect.size[0] as u32,
                rect.size[1] as u32,
            )
            .to_image();

            sub_buffer
                .save(format!(
                    "../../assets/tests/tilesets/{}#{}.png",
                    tileset.uuid, index
                ))
                .unwrap();
        }
    }
}
