use super::runtime::Interface;
use gridthorn::ui::{UiControl, UiNodeId, UiRoute, UiSelection};
use gridthorn::{InputEvent, TextInputEvent};

impl Interface {
    pub(super) fn editor_step(
        &mut self,
        frame: u32,
        preedit: bool,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        let mut route = self.focus_editor()?;
        if frame <= 10 {
            return Ok(route);
        }
        let layout = self.layout.as_ref().ok_or("editor workload needs layout")?;
        if preedit {
            let event = if frame == 120 {
                TextInputEvent::CompositionCancelled
            } else {
                let text = if frame.is_multiple_of(2) {
                    "にほん"
                } else {
                    "العربية e\u{301}"
                };
                TextInputEvent::Composition {
                    text: text.into(),
                    cursor: Some((text.len(), text.len())),
                }
            };
            let injected =
                self.router
                    .route_events(&mut self.tree, layout, &[InputEvent::Text(event)])?;
            route.platform.extend(injected.platform);
            route.effects.extend(injected.effects);
        } else {
            let UiControl::TextField { value, .. } =
                &self.tree.node(UiNodeId(7)).ok_or("missing editor")?.control
            else {
                return Err("selection workload needs text field".into());
            };
            self.router.select(
                &self.tree,
                UiSelection {
                    anchor: 0,
                    caret: if frame.is_multiple_of(2) {
                        value.len()
                    } else {
                        0
                    },
                },
            )?;
        }
        Ok(route)
    }
}

#[cfg(test)]
#[path = "test/editor_workload.rs"]
mod test;
