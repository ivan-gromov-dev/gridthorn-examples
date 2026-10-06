use super::super::{composition, model::Request, presentation::Menu};
use gridthorn::{
    GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsAdapters,
    GraphicsBackend, WindowViewport,
};
fn adapter(api: GraphicsBackend, name: &str) -> GraphicsAdapter {
    GraphicsAdapter {
        key: GraphicsAdapterKey {
            backend: api,
            vendor: 1,
            device: 2,
            name: name.into(),
        },
        compatibility: GraphicsAdapterCompatibility::Compatible,
        software: false,
    }
}
#[test]
fn routes_separate_card_and_api_choices_and_saves_without_switching() {
    let mut menu = Menu::new().unwrap();
    let vulkan = adapter(GraphicsBackend::Vulkan, "GPU");
    let dx = adapter(GraphicsBackend::Direct3D12, "GPU");
    menu.model.adapters = Some(GraphicsAdapters {
        adapters: vec![vulkan.clone(), dx.clone()],
        selected: vulkan.key.clone(),
    });
    assert_eq!(menu.model.devices().len(), 1);
    menu.rebuild().unwrap();
    let viewport = WindowViewport {
        width: 1100,
        height: 2400,
    };
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::ADAPTERS, 1, 1.0).unwrap(), []);
    assert!(menu.model.graphics_selection.device.is_some());
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::RENDER_API, 2, 1.0).unwrap(), []);
    assert_eq!(
        menu.model.graphics_selection.api,
        Some(GraphicsBackend::Direct3D12)
    );
    assert_eq!(menu.model.adapters.as_ref().unwrap().selected, vulkan.key);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(
        menu.click(composition::SAVE_ADAPTER, 0, 1.0).unwrap(),
        [Request::SaveAdapter]
    );
}
#[test]
fn changing_card_filters_apis_and_resets_an_unsupported_staged_api() {
    let mut menu = Menu::new().unwrap();
    let vk = adapter(GraphicsBackend::Vulkan, "GPU A");
    let dx = adapter(GraphicsBackend::Direct3D12, "GPU B");
    menu.model.adapters = Some(GraphicsAdapters {
        adapters: vec![vk.clone(), dx],
        selected: vk.key,
    });
    menu.model.choose_adapter(Some(1));
    menu.model.choose_api(Some(1));
    assert_eq!(
        menu.model.graphics_selection.api,
        Some(GraphicsBackend::Vulkan)
    );
    menu.model.choose_adapter(Some(2));
    assert_eq!(menu.model.rendering_apis(), [GraphicsBackend::Direct3D12]);
    assert_eq!(menu.model.graphics_selection.api, None);
    menu.model.choose_adapter(Some(0));
    assert_eq!(menu.model.rendering_apis().len(), 2);
}
#[test]
fn rejects_ambiguous_api_and_keeps_scroll_during_rebuild() {
    let mut menu = Menu::new().unwrap();
    let gpu = adapter(GraphicsBackend::Vulkan, "GPU");
    menu.model.adapters = Some(GraphicsAdapters {
        adapters: vec![gpu.clone(), gpu.clone()],
        selected: gpu.key,
    });
    menu.model.choose_adapter(Some(1));
    menu.model.choose_api(Some(1));
    assert_eq!(menu.model.graphics_selection.api, None);
    assert!(menu.model.adapter_status.contains("однозначно"));
    menu.rebuild().unwrap();
    menu.prepare(
        WindowViewport {
            width: 1100,
            height: 860,
        },
        1.0,
    )
    .unwrap();
    menu.tree
        .command(
            gridthorn::ui::UiNodeId(5),
            gridthorn::ui::UiCommand::ScrollTo([0.0, 300.0]),
        )
        .unwrap();
    menu.rebuild().unwrap();
    assert_eq!(
        menu.tree
            .node(gridthorn::ui::UiNodeId(5))
            .unwrap()
            .scroll_offset,
        [0.0, 300.0]
    );
}
