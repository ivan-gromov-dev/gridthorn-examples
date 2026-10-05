use super::runtime::Interface;
use gridthorn::ui::{UiCommand, UiEffect, UiNodeId, UiRoute};

impl Interface {
    pub(super) fn window_step(
        &mut self,
        frame: u32,
        animated: bool,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        let frame = frame.saturating_sub(1) % 120 + 1;
        if frame == 1 {
            self.tree
                .command(UiNodeId(4), UiCommand::SetChecked(animated))?;
        }
        let action = match frame {
            11 | 101 => Some(3),
            26 => Some(201),
            41 => Some(301),
            56 => Some(203),
            71 => Some(402),
            86 | 116 => Some(204),
            _ => None,
        };
        Ok(UiRoute {
            effects: action
                .map(|id| vec![(UiNodeId(id), UiEffect::Activated)])
                .unwrap_or_default(),
            ..UiRoute::default()
        })
    }
}

#[cfg(test)]
#[path = "test/window_workload.rs"]
mod test;
