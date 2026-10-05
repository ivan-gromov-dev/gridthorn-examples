use crate::game::model::{Arena, GEMS, Session, WALLS};
use gridthorn::prelude::*;
use std::time::Duration;

pub(crate) struct Assets {
    texture: TextureAsset,
    animation: AnimationPlayer,
}

impl Assets {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let texture = TextureAsset::load(crate::assets::directory()?.join("atlas.ppm"))?;
        let animation = AnimationPlayer::new(AnimationClip::new(
            vec![region(0), region(1)],
            Duration::from_millis(180),
            AnimationPlayback::Loop,
        )?);
        Ok(Self { texture, animation })
    }
}

fn region(index: u8) -> SpriteRegion {
    SpriteRegion::new(
        [f32::from(index) / 4.0, 0.0],
        [f32::from(index + 1) / 4.0, 1.0],
    )
    .expect("atlas region")
}

fn text(value: impl Into<String>, position: [f32; 2], scale: f32) -> UiPrimitive {
    TextLabel::new(value, position, scale, Color::rgb(0.85, 0.95, 1.0))
        .expect("label geometry")
        .into()
}

pub(crate) fn register(schedules: &mut ScheduleBuilder, mut assets: Assets) {
    schedules.add_system(ScheduleStage::Render, move |world| {
        let active = world
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .unwrap();
        let (entity, hovered) = world
            .read_resource(|s: &Session| (s.entity, s.hovered))
            .unwrap();
        let arena: Option<Arena> = entity.and_then(|id| world.read_component(id, Clone::clone));
        let elapsed = world
            .read_resource(|t: &FrameTiming| t.frame_elapsed())
            .unwrap_or_default();
        if active == "play" {
            assets.animation.play();
        } else {
            assets.animation.pause();
        }
        assets.animation.advance(elapsed);
        let mut sprites = Vec::new();
        if let Some(arena) = &arena {
            for wall in WALLS {
                sprites.push(
                    TexturedSprite::new(
                        [f32::from(wall[0]), f32::from(wall[1])],
                        [48.0, 130.0],
                        assets.texture.clone(),
                    )
                    .with_region(region(3)),
                );
            }
            for (index, gem) in GEMS.iter().enumerate() {
                if !arena.collected[index] {
                    sprites.push(
                        TexturedSprite::new(
                            [f32::from(gem[0]), f32::from(gem[1])],
                            [24.0, 24.0],
                            assets.texture.clone(),
                        )
                        .with_region(region(2)),
                    );
                }
            }
            sprites.push(
                TexturedSprite::new(
                    [f32::from(arena.position[0]), f32::from(arena.position[1])],
                    [28.0, 28.0],
                    assets.texture.clone(),
                )
                .with_region(assets.animation.region()),
            );
        }
        let ui = interface(&active, hovered, arena.as_ref());
        let mut frame = RenderFrame::new(
            Camera2d::new([0.0, 0.0], 540.0),
            vec![Sprite::new(
                [0.0, 0.0],
                [760.0, 440.0],
                Color::rgb(0.035, 0.09, 0.12),
            )],
        )
        .with_textured_sprites(sprites)
        .with_ui(ui);
        if let Some(timing) = world.read_resource(|t: &FrameTiming| *t) {
            frame = frame.with_timing_overlay(TimingOverlay::new(
                timing.frame_elapsed(),
                timing.fixed_steps(),
                timing.accumulated_lag(),
                timing.overloaded(),
            ));
        }
        world.insert_resource(frame);
    });
}

fn interface(active: &str, hovered: bool, arena: Option<&Arena>) -> Vec<UiPrimitive> {
    let mut ui = vec![text("CRYSTAL TRAIL", [24.0, 24.0], 3.0)];
    let label = match active {
        "menu" => "COLLECT ALL 4 CRYSTALS",
        "paused" => "PAUSED",
        "won" => "YOU WIN! ALL CRYSTALS FOUND",
        _ => "WASD / ARROWS MOVE - SPACE PAUSE - ESC QUIT",
    };
    ui.push(text(label, [24.0, 64.0], 2.0));
    if active != "play" {
        ui.push(
            UiRect::new(
                [24.0, 110.0],
                [240.0, 44.0],
                if hovered {
                    Color::rgb(0.2, 0.5, 0.65)
                } else {
                    Color::rgb(0.1, 0.25, 0.4)
                },
            )
            .expect("panel geometry")
            .into(),
        );
        ui.push(text(
            if active == "paused" {
                "RESUME / ENTER"
            } else {
                "PLAY / ENTER"
            },
            [36.0, 124.0],
            2.0,
        ));
    }
    if let Some(arena) = arena {
        ui.push(text(
            format!(
                "CRYSTALS {}/4",
                arena.collected.iter().filter(|v| **v).count()
            ),
            [24.0, 490.0],
            2.0,
        ));
    }
    ui
}
