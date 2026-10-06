use gridthorn::{
    ApplicationRuntime, ExitRequest, GraphicsAdapters, GraphicsSelection, ScheduleBuilder,
    ScheduleStage, graphics_devices,
};

pub(crate) fn runtime(expected: GraphicsSelection) -> ApplicationRuntime {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        let adapters = world
            .read_resource(|adapters: &GraphicsAdapters| adapters.clone())
            .expect("renderer inventory before startup");
        if let Some(api) = expected.api {
            assert_eq!(adapters.selected.backend, api);
        }
        if let Some(device) = &expected.device {
            assert!(
                graphics_devices(&adapters.adapters)
                    .iter()
                    .any(|candidate| &candidate.key == device
                        && candidate
                            .apis
                            .iter()
                            .any(|api| api.key == adapters.selected))
            );
        }
        println!("adapter_smoke: selected {:?}", adapters.selected);
    });
    let mut frames = 0;
    schedules.add_system(ScheduleStage::Update, move |world| {
        frames += 1;
        if frames == 3 {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.world().insert_resource(ExitRequest::default());
    runtime
}
