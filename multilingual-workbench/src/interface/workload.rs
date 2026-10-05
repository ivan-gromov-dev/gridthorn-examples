use super::runtime::Interface;
use gridthorn::ui::{UiControl, UiNavigation, UiNodeId, UiRoute, UiSelection};
use gridthorn::{ButtonState, CursorPosition, InputEvent, MouseButton, TextInputEvent};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SmokeWorkload {
    Mixed,
    Idle,
    Editing,
    Slider,
    Locale,
    Scroll,
    Windows,
    Animation,
    Selection,
    Preedit,
}

impl Interface {
    pub(super) fn locale_step(
        &mut self,
        frame: u32,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        if frame <= 10 {
            return Ok(UiRoute::default());
        }
        let UiControl::List { selected, .. } = self
            .tree
            .node(UiNodeId(6))
            .ok_or("locale workload needs language list")?
            .control
        else {
            return Err("locale workload needs list control".into());
        };
        let next = (selected.unwrap_or(0) + 1) % super::messages::LANGUAGES.len();
        self.tree
            .command(UiNodeId(6), gridthorn::ui::UiCommand::Select(Some(next)))?;
        Ok(UiRoute {
            effects: vec![(UiNodeId(6), gridthorn::ui::UiEffect::Changed)],
            ..UiRoute::default()
        })
    }

    pub(super) fn slider_step(
        &mut self,
        frame: u32,
        dpi: f64,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        if frame <= 10 {
            return Ok(UiRoute::default());
        }
        let layout = self.layout.as_ref().ok_or("slider workload needs layout")?;
        let content = layout
            .placement(UiNodeId(5))
            .ok_or("slider workload needs placement")?
            .content;
        let ratio = if frame.is_multiple_of(2) { 0.2 } else { 0.8 };
        let mut events = vec![InputEvent::CursorMoved(CursorPosition {
            x: f64::from(content.position[0] + content.size[0] * ratio) * dpi,
            y: f64::from(content.position[1] + content.size[1] * 0.5) * dpi,
        })];
        if frame == 11 {
            events.push(InputEvent::MouseButton {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
            });
        } else if frame == 120 {
            events.push(InputEvent::MouseButton {
                button: MouseButton::Left,
                state: ButtonState::Released,
            });
        }
        Ok(self.router.route_events(&mut self.tree, layout, &events)?)
    }

    pub(super) fn focus_editor(&mut self) -> Result<UiRoute, Box<dyn std::error::Error>> {
        let layout = self
            .layout
            .as_ref()
            .ok_or("editing workload needs prepared layout")?;
        let mut platform = Vec::new();
        for _ in 0..16 {
            if self.router.focused() == Some(UiNodeId(7)) {
                break;
            }
            platform.extend(
                self.router
                    .navigate(&mut self.tree, layout, UiNavigation::Next)?
                    .platform,
            );
        }
        if self.router.focused() != Some(UiNodeId(7)) {
            return Err("editing workload could not focus editor".into());
        }
        Ok(UiRoute {
            platform,
            ..UiRoute::default()
        })
    }

    pub(super) fn editing_step(
        &mut self,
        frame: u32,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        let focused = self.focus_editor()?;
        if frame <= 10 {
            return Ok(focused);
        }
        let layout = self
            .layout
            .as_ref()
            .ok_or("editing workload needs prepared layout")?;
        let UiControl::TextField { value, .. } =
            &self.tree.node(UiNodeId(7)).ok_or("missing editor")?.control
        else {
            return Err("editing workload needs text field".into());
        };
        self.router.select(
            &self.tree,
            UiSelection {
                anchor: 0,
                caret: value.len(),
            },
        )?;
        let mut route = self.router.route_events(
            &mut self.tree,
            layout,
            &[InputEvent::Text(TextInputEvent::Commit(format!(
                "Привет مرحبًا 日本語 e\u{301} {frame}"
            )))],
        )?;
        route.platform.extend(focused.platform);
        Ok(route)
    }
}

#[cfg(test)]
#[path = "test/workload.rs"]
mod test;
