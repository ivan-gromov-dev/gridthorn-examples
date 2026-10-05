use gridthorn::ui::{UiControl, UiNode, UiNodeId, UiRouter, UiSelection, UiTree, UiVisualState};

/// Presentation inputs that can change in this example; authored styles stay fixed.
#[derive(PartialEq)]
pub(super) struct LayoutState {
    nodes: Vec<NodeState>,
    focus: Option<UiNodeId>,
    selection: Option<UiSelection>,
    preedit: String,
    composition_cursor: Option<(usize, usize)>,
    layers: Vec<UiNodeId>,
    metrics: ([f32; 2], f32),
}

#[derive(PartialEq)]
struct NodeState {
    id: UiNodeId,
    control: UiControl,
    visual: UiVisualState,
    scroll: [f32; 2],
    offset: [f32; 2],
}

impl LayoutState {
    pub(super) fn capture(tree: &UiTree, router: &UiRouter, metrics: ([f32; 2], f32)) -> Self {
        let mut nodes = Vec::new();
        collect(tree.root(), &mut nodes);
        Self {
            nodes,
            focus: router.focused(),
            selection: router.selection(tree),
            preedit: router.preedit().into(),
            composition_cursor: router.composition_cursor(),
            layers: router.open_layers(),
            metrics,
        }
    }
}

fn collect(node: &UiNode, output: &mut Vec<NodeState>) {
    output.push(NodeState {
        id: node.id,
        control: node.control.clone(),
        visual: node.visual,
        scroll: node.scroll_offset,
        offset: node.style.offset,
    });
    for child in &node.children {
        collect(child, output);
    }
}
