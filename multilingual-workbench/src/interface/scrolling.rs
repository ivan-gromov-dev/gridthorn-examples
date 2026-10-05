use super::runtime::Interface;
use gridthorn::ui::{UiNodeId, UiRoute};
use gridthorn::{CursorPosition, InputEvent, ScrollPhase, WheelDelta};

impl Interface {
    pub(super) fn scroll_step(
        &mut self,
        frame: u32,
        dpi: f64,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        if frame <= 10 {
            return Ok(UiRoute::default());
        }
        let layout = self.layout.as_ref().ok_or("scroll workload needs layout")?;
        let panel = layout
            .placement(UiNodeId(0))
            .ok_or("scroll workload needs panel")?;
        if panel.content_extent[1] <= panel.content.size[1] {
            return Err("scroll workload needs overflowing viewport".into());
        }
        let delta = if frame.is_multiple_of(2) { 96.0 } else { -96.0 };
        Ok(self.router.route_events(
            &mut self.tree,
            layout,
            &[
                InputEvent::CursorMoved(CursorPosition {
                    x: f64::from(panel.content.position[0] + 2.0) * dpi,
                    y: f64::from(panel.content.position[1] + 2.0) * dpi,
                }),
                InputEvent::MouseWheel {
                    delta: WheelDelta::Pixels {
                        x: 0.0,
                        y: delta * dpi,
                    },
                    phase: ScrollPhase::Moved,
                },
            ],
        )?)
    }
}

#[cfg(test)]
#[path = "test/scrolling.rs"]
mod test;
