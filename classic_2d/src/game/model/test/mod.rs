use super::*;

#[test]
fn walls_block_movement_and_pickups_are_once_only() {
    let mut arena = Arena {
        position: [-139, 0],
        ..Default::default()
    };
    arena.step([1, 0]);
    assert_eq!(arena.position, [-139, 0]);
    arena.position = GEMS[0];
    assert_eq!(arena.step([0, 0]), 1);
    assert_eq!(arena.step([0, 0]), 0);
}

#[test]
fn movement_clamps_to_arena() {
    let mut arena = Arena {
        position: [360, 200],
        ..Default::default()
    };
    arena.step([1, 1]);
    assert_eq!(arena.position, [360, 200]);
}
