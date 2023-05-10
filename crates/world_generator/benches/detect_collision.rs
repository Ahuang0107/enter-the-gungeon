#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn detect_collision(b: &mut Bencher) {
    let level = world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
    b.iter(|| level.contains_floor([100, -200]));
}
