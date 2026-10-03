use crate::game::session::{Session, button_visible, layout_bounds};
use gridthorn::{
    Color, SimulationControl, SpriteRegion, TextureAsset, TexturedSprite, WindowViewport,
};

pub fn region(rect: [u16; 4], texture: &TextureAsset) -> SpriteRegion {
    let [x, y, w, h] = rect;
    let dimensions = texture.dimensions();
    #[allow(clippy::cast_precision_loss)]
    let scale = [dimensions[0] as f32, dimensions[1] as f32];
    SpriteRegion::new(
        [f32::from(x) / scale[0], f32::from(y) / scale[1]],
        [f32::from(x + w) / scale[0], f32::from(y + h) / scale[1]],
    )
    .expect("atlas rectangle")
}

#[allow(clippy::cast_precision_loss)]
fn screen(
    s: &Session,
    texture: &TextureAsset,
    rect: [u16; 4],
    position: [f32; 2],
    size: [f32; 2],
    viewport: WindowViewport,
) -> TexturedSprite {
    let scale = s.height / viewport.height.max(1) as f32;
    let center = [
        s.camera[0] + (position[0] + size[0] * 0.5 - viewport.width as f32 * 0.5) * scale,
        s.camera[1] + (position[1] + size[1] * 0.5 - viewport.height as f32 * 0.5) * scale,
    ];
    TexturedSprite::new(center, [size[0] * scale, size[1] * scale], texture.clone())
        .with_region(region(rect, texture))
}

#[allow(clippy::cast_precision_loss)]
pub fn extract(
    s: &Session,
    texture: &TextureAsset,
    active: &str,
    control: SimulationControl,
    viewport: WindowViewport,
) -> Vec<TexturedSprite> {
    let (scale, width, height) = crate::game::session::ui_layout(viewport);
    let mut sprites = Vec::new();
    let fill = [210, 230, 2, 2];
    for (position, size) in [
        ([0.0, 0.0], [width, 112.0]),
        ([0.0, 112.0], [236.0, (height - 174.0).max(1.0)]),
        ([width - 292.0, 112.0], [292.0, (height - 174.0).max(1.0)]),
        ([0.0, height - 62.0], [width, 62.0]),
    ] {
        sprites.extend(framed(
            s,
            texture,
            [459, 126, 412, 235],
            position.map(|v| v * scale),
            size.map(|v| v * scale),
            10.0 * scale,
            viewport,
        ));
    }
    if active != "play" {
        sprites.push(
            screen(
                s,
                texture,
                fill,
                [0.0, 112.0 * scale],
                [width * scale, (height - 174.0).max(1.0) * scale],
                viewport,
            )
            .with_tint(Color::rgba(0.5, 0.7, 0.7, 0.82)),
        );
        sprites.extend(framed(
            s,
            texture,
            [459, 126, 412, 235],
            [(width - 360.0) * 0.5 * scale, 130.0 * scale],
            [360.0 * scale, 500.0 * scale],
            24.0 * scale,
            viewport,
        ));
    }
    for index in 0..35 {
        if !button_visible(index, active, s) {
            continue;
        }
        let (position, size) = layout_bounds(index, viewport);
        let rect = if s.hovered[index] {
            [1338, 113, 434, 263]
        } else if super::interface::chosen(index, s, control) {
            [884, 109, 446, 274]
        } else {
            [459, 126, 412, 235]
        };
        sprites.extend(framed(
            s,
            texture,
            rect,
            position,
            size,
            6.0 * scale,
            viewport,
        ));
    }
    for (rect, x) in [
        ([17, 459, 413, 389], 114.0),
        ([470, 468, 416, 381], 286.0),
        ([903, 479, 414, 370], 450.0),
        ([1342, 396, 417, 459], 798.0),
    ] {
        sprites.push(screen(
            s,
            texture,
            rect,
            [x * scale, 17.0 * scale],
            [34.0 * scale, 34.0 * scale],
            viewport,
        ));
    }
    sprites
}

fn framed(
    session: &Session,
    texture: &TextureAsset,
    rect: [u16; 4],
    position: [f32; 2],
    size: [f32; 2],
    edge: f32,
    viewport: WindowViewport,
) -> Vec<TexturedSprite> {
    let [left, top, width, height] = rect;
    let source_x = [left, left + 90, left + width - 90, left + width];
    let source_y = [top, top + 70, top + height - 60, top + height];
    let target_x = [
        position[0],
        position[0] + edge,
        position[0] + size[0] - edge,
        position[0] + size[0],
    ];
    let target_y = [
        position[1],
        position[1] + edge,
        position[1] + size[1] - edge,
        position[1] + size[1],
    ];
    let mut sprites = Vec::new();
    for row in 0..3 {
        for column in 0..3 {
            sprites.push(screen(
                session,
                texture,
                [
                    source_x[column],
                    source_y[row],
                    source_x[column + 1] - source_x[column],
                    source_y[row + 1] - source_y[row],
                ],
                [target_x[column], target_y[row]],
                [
                    target_x[column + 1] - target_x[column],
                    target_y[row + 1] - target_y[row],
                ],
                viewport,
            ));
        }
    }
    sprites
}
