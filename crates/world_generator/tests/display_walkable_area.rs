/// 可视化获得的walkable area
#[test]
fn display_walkable_area() {
    let level = world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;
    for room in level.rooms.iter() {
        let rect = room.get_rect();
        for [x, y] in vec![rect.min, rect.get_max()] {
            if min_x.is_none() || min_x.unwrap() > x {
                min_x = Some(x);
            }
            if max_x.is_none() || max_x.unwrap() < x {
                max_x = Some(x);
            }
            if min_y.is_none() || min_y.unwrap() > y {
                min_y = Some(y);
            }
            if max_y.is_none() || max_y.unwrap() < y {
                max_y = Some(y);
            }
        }
    }
    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();
    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();
    let width = (max_x - min_x) as u32;
    let height = (max_y - min_y).abs() as u32;
    let mut output = image::ImageBuffer::new(width, height);
    for room in level.rooms.iter() {
        let offset = room.world_pos;
        for area in room.walkable_area.iter() {
            let left_top = [area.min[0] + offset[0], area.min[1] + offset[1]];
            let left = (left_top[0] - min_x) as u32;
            let top = (left_top[1] - min_y).abs() as u32;
            let right_bottom = [
                left_top[0] + area.size[0] as f32,
                left_top[1] - area.size[1] as f32,
            ];
            let right = (right_bottom[0] - min_x) as u32;
            let bottom = (right_bottom[1] - min_y).abs() as u32;
            for x in left..right {
                for y in (height - top)..(height - bottom) {
                    output.put_pixel(x, y, image::Rgba::from([0_u8, 0, 0, 255]));
                }
            }
        }
    }
    output
        .save("../../assets/tests/demo_output_walkable_area.png")
        .unwrap();
}
