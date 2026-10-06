use gridthorn::{
    InputState,
    controller::{ControllerId, ControllerInfo},
};
/// Live inventory and game-owned selection, independent of UI row positions.
#[derive(Default)]
pub(super) struct Devices {
    pub(super) controllers: Vec<ControllerInfo>,
    pub(super) selected: Option<ControllerId>,
    pub(super) feedback: String,
}
impl Devices {
    pub(super) fn sync(&mut self, input: &InputState) -> bool {
        let next: Vec<_> = input
            .controllers()
            .map(|state| state.info().clone())
            .collect();
        let changed = self.controllers != next;
        self.controllers = next;
        if self
            .selected
            .is_some_and(|id| !self.controllers.iter().any(|info| info.id == id))
        {
            self.selected = None;
        }
        changed
    }
    pub(super) fn items(&self) -> Vec<String> {
        std::iter::once("Клавиатура и мышь".into())
            .chain(
                self.controllers
                    .iter()
                    .map(|info| format!("{} · #{}", info.name, info.id.0)),
            )
            .collect()
    }
    pub(super) fn row(&self) -> usize {
        self.selected
            .and_then(|id| self.controllers.iter().position(|info| info.id == id))
            .map_or(0, |index| index + 1)
    }
    pub(super) fn select(&mut self, row: usize) {
        self.selected = row
            .checked_sub(1)
            .and_then(|index| self.controllers.get(index))
            .map(|info| info.id);
    }
}
