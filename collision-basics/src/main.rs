use gridthorn::contact;
use gridthorn::prelude::{Aabb2d, Circle2d, Collider2d, Vec2};

fn main() {
    let player: Collider2d = Circle2d::new(Vec2::new(2.5, 0.0), 1.0)
        .expect("player collider should be valid")
        .into();
    let wall: Collider2d = Aabb2d::new(Vec2::new(3.0, 0.0), Vec2::new(0.5, 4.0))
        .expect("wall collider should be valid")
        .into();
    let collision = contact(player, wall).expect("player should overlap the wall");

    println!(
        "collision normal=({:.1}, {:.1}) penetration={:.1}",
        collision.normal().x,
        collision.normal().y,
        collision.penetration()
    );
}
