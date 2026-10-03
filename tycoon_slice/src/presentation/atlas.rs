use gridthorn::{SpriteRegion, TextureAsset};

/// Atlas source rectangles are explicit to isolate authored silhouettes and prevent bleeding.
pub fn region(index: u8) -> SpriteRegion {
    let [x, y, w, h] = match index {
        0 => [0.0, 96.0, 313.0, 206.0],
        1 => [313.0, 96.0, 314.0, 206.0],
        2 => [627.0, 96.0, 313.0, 206.0],
        3 => [940.0, 96.0, 314.0, 206.0],
        4 => [0.0, 302.0, 313.0, 334.0],
        5 => [313.0, 302.0, 314.0, 334.0],
        6 => [627.0, 302.0, 313.0, 334.0],
        7 => [940.0, 302.0, 314.0, 334.0],
        8 => [0.0, 636.0, 313.0, 336.0],
        9 => [313.0, 636.0, 325.0, 336.0],
        10 => [738.0, 710.0, 126.0, 218.0],
        11 => [1050.0, 710.0, 126.0, 218.0],
        12 => [0.0, 990.0, 313.0, 221.0],
        13 => [313.0, 990.0, 314.0, 221.0],
        14 => [627.0, 998.0, 313.0, 216.0],
        _ => [940.0, 998.0, 314.0, 216.0],
    };
    SpriteRegion::new(
        [x / 1254.0, y / 1254.0],
        [(x + w) / 1254.0, (y + h) / 1254.0],
    )
    .expect("atlas rectangle")
}

pub fn sprite(
    texture: &TextureAsset,
    index: u8,
    position: [f32; 2],
    size: [f32; 2],
) -> gridthorn::TexturedSprite {
    gridthorn::TexturedSprite::new(position, size, texture.clone()).with_region(region(index))
}
