#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn serialize_single_level(b: &mut Bencher) {
    b.iter(|| {
        let level =
            world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
        return level.rooms.len();
    });
}
