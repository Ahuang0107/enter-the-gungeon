#[test]
fn output_tileset_split_png() {
    let level = world_generator::demo_level_model();
    for tileset in level.tilesets.iter() {
        let mut dynamic_image = image::open(format!("../../assets/{}", tileset.src)).unwrap();
        let buffer = dynamic_image.as_mut_rgba8().unwrap();
        for (index, rect) in tileset.tiles.iter() {
            let sub_buffer = image::imageops::crop(
                buffer,
                rect.min[0] as u32,
                rect.min[1] as u32,
                rect.size[0],
                rect.size[1],
            )
            .to_image();
            sub_buffer
                .save(format!("../../assets/tests/{}#{}.png", tileset.uuid, index))
                .unwrap();
        }
    }
}
