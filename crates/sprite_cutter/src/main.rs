use image::{open, GenericImage, ImageBuffer, Rgba};

const BACKGROUND: [u8; 4] = [153, 155, 189, 255];
const TRANSPARENT: [u8; 4] = [255, 255, 255, 0];

fn main() {
    let mut img = open("assets/art/character/The Marine.png").unwrap();
    for (index, &x) in [497, 516, 535, 554].iter().enumerate() {
        let mut sub = img.sub_image(x, 39, 17, 19).to_image();
        for rgba in sub.pixels_mut() {
            if rgba.0 == BACKGROUND {
                rgba.0 = TRANSPARENT;
            }
        }
        scale_to(sub, 32, 32)
            .save(format!("assets/tests/characters/idle-front-{index}.png"))
            .unwrap();
        let mut sub = img.sub_image(x, 89, 17, 19).to_image();
        for rgba in sub.pixels_mut() {
            if rgba.0 == BACKGROUND {
                rgba.0 = TRANSPARENT;
            }
        }
        scale_to(sub, 32, 32)
            .save(format!("assets/tests/characters/idle-back-{index}.png"))
            .unwrap();
        let mut sub = img.sub_image(x, 114, 17, 19).to_image();
        for rgba in sub.pixels_mut() {
            if rgba.0 == BACKGROUND {
                rgba.0 = TRANSPARENT;
            }
        }
        scale_to(sub, 32, 32)
            .save(format!("assets/tests/characters/idle-left-{index}.png"))
            .unwrap();
    }
    for (index, &x) in [497, 515, 533, 551].iter().enumerate() {
        let mut sub = img.sub_image(x, 64, 16, 19).to_image();
        for rgba in sub.pixels_mut() {
            if rgba.0 == BACKGROUND {
                rgba.0 = TRANSPARENT;
            }
        }
        scale_to(sub, 32, 32)
            .save(format!("assets/tests/characters/idle-right-{index}.png"))
            .unwrap();
    }
}

fn scale_to(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let old_width = img.width();
    let old_height = img.height();
    let left_start = (width - old_width) / 2;
    let top_start = (height - old_height) / 2;
    let mut result = image::ImageBuffer::new(width, height);
    for x in 0..old_width {
        for y in 0..old_height {
            let pixel = img.get_pixel(x, y);
            result.put_pixel(left_start + x, top_start + y, *pixel);
        }
    }
    result
}
