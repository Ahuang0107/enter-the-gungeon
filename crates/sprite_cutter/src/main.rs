use std::cmp::max;

use image::{open, DynamicImage, GenericImage, ImageBuffer, Rgba};

const BACKGROUND: [u8; 4] = [153, 155, 189, 255];

fn main() {
    std::fs::create_dir_all("assets/tests/characters").unwrap();
    let mut img = open("assets/art/character/The Marine.png").unwrap();

    // for (index, sub) in vec![
    //     take(&mut img, (628, 150), (17, 20)),
    //     take(&mut img, (645, 150), (17, 20)),
    //     take(&mut img, (662, 150), (18, 20)),
    //     take(&mut img, (680, 150), (17, 20)),
    //     take(&mut img, (698, 150), (17, 20)),
    //     take(&mut img, (715, 150), (17, 20)),
    // ]
    // .into_iter()
    // .map(|i| scale_to(i, 32, 32))
    // .enumerate()
    // {
    //     sub.save(format!("assets/tests/characters/walking-front-{index}.png"))
    //         .unwrap();
    // }

    for (index, sub) in vec![
        take(&mut img, (628, 150), (17, 20)),
        take(&mut img, (645, 150), (17, 20)),
        take(&mut img, (662, 150), (18, 20)),
        take(&mut img, (680, 150), (17, 20)),
        take(&mut img, (698, 150), (17, 20)),
        take(&mut img, (715, 150), (17, 20)),
    ]
    .into_iter()
    .map(|i| scale_to(i, 32, 32))
    .enumerate()
    {
        sub.save(format!("assets/tests/characters/walking-front-{index}.png"))
            .unwrap();
    }
}

fn take(
    img: &mut DynamicImage,
    min: (u32, u32),
    size: (u32, u32),
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (x, y) = min;
    let (width, height) = size;
    img.sub_image(x, y, width, height).to_image()
}

fn split(
    mut img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    columns: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let width = img.width();
    let height = img.height();
    let sub_width = width / columns;
    let mut result = vec![];
    for col in 0..columns {
        result.push(
            img.sub_image(col * sub_width, 0, sub_width, height)
                .to_image(),
        )
    }
    result
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
            if pixel.0 != BACKGROUND {
                result.put_pixel(left_start + x, top_start + y, *pixel);
            }
        }
    }
    result
}

fn merge_h(images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut width = 0;
    let mut height = 0;
    for img in images.iter() {
        width += img.width();
        height = max(height, img.height());
    }
    let mut result = image::ImageBuffer::new(width, height);
    let mut left = 0;
    for img in images.iter() {
        for x in 0..img.width() {
            for y in 0..img.height() {
                let pixel = img.get_pixel(x, y);
                result.put_pixel(left + x, y, *pixel);
            }
        }
        left += img.width();
    }
    result
}

fn merge_v(images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut width = 0;
    let mut height = 0;
    for img in images.iter() {
        height += img.height();
        width = max(width, img.width());
    }
    let mut result = image::ImageBuffer::new(width, height);
    let mut top = 0;
    for img in images.iter() {
        for x in 0..img.width() {
            for y in 0..img.height() {
                let pixel = img.get_pixel(x, y);
                result.put_pixel(x, top + y, *pixel);
            }
        }
        top += img.height();
    }
    result
}
