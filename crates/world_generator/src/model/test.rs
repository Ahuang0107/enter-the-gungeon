#[test]
fn check_room_contains() {
    use crate::{Rect, RoomModel};
    let room = RoomModel {
        world_pos: [-16, 48],
        size: [368, 368],
        walkable_area: vec![Rect {
            min: [16, -48],
            size: [16, 16],
        }],
        ..Default::default()
    };
    // not in room
    for pos in vec![[-17, 48], [352, -321]] {
        assert!(!room.in_room(pos), "{:?} in room", pos);
    }
    // in room
    for pos in vec![[-16, 48], [352, -320]] {
        assert!(room.in_room(pos), "{:?} not in room", pos);
    }
    // not contains
    for pos in vec![[-1, 0], [0, 1], [0, -17], [17, 0], [17, -17]] {
        assert!(!room.contains(pos), "contains {:?}", pos);
    }
    // contains
    for pos in vec![[0, 0], [16, 0], [0, -16], [16, -16]] {
        assert!(room.contains(pos), "not contains {:?}", pos);
    }
}

#[test]
fn check_rect_contains() {
    use crate::Rect;
    let rect = Rect {
        min: [96, -96],
        size: [16, 16],
    };
    // not contains
    for pos in vec![
        // higher
        [95, -95],
        [96, -95],
        [104, -95],
        [112, -95],
        [113, -95],
        [120, -95],
        // lower
        [95, -113],
        [96, -113],
        [104, -113],
        [112, -113],
        [113, -113],
        [120, -113],
        // lefter
        [95, -95],
        [95, -96],
        [95, -104],
        [95, -112],
        [95, -120],
        // righter
        [113, -95],
        [113, -96],
        [113, -104],
        [113, -112],
        [113, -120],
    ] {
        assert!(!rect.contains(pos), "contains {:?}", pos);
    }
    // contains
    for pos in vec![
        [96, -96],
        [97, -96],
        [97, -97],
        [96, -97],
        [104, -104],
        [111, -112],
        [111, -111],
        [112, -111],
        [112, -112],
    ] {
        assert!(rect.contains(pos), "not contains {:?}", pos);
    }
}
