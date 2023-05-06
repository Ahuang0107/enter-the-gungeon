#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn detect_collision(b: &mut Bencher) {
    let level = world_generator::LevelModel::from("../../assets/levels/demo_output.json").unwrap();
    b.iter(|| {
        let mut result = false;
        let pos = [100, -200];
        for room in level.rooms.iter() {
            if room.contains(pos) {
                result = true;
            }
        }
        return result;
    });
}
