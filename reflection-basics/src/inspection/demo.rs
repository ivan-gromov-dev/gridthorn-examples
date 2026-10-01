use gridthorn::{
    FieldMetadata, Reflect, ReflectValue, ReflectionRegistry, ScheduleBuilder, ValueKind,
};

struct Health(u64);

impl Reflect for Health {
    const TYPE_NAME: &'static str = "example.health";
    const FIELDS: &'static [FieldMetadata] = &[FieldMetadata {
        name: "points",
        kind: ValueKind::Unsigned,
    }];

    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Unsigned(self.0)]
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = ReflectionRegistry::default();
    registry.register_component::<Health>()?;
    registry.register_resource::<Health>()?;
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    let player = world.spawn(Health(100));
    world.insert_resource(Health(500));
    assert_eq!(
        registry.inspect_component(&world, player, Health::TYPE_NAME)?,
        Some(vec![ReflectValue::Unsigned(100)])
    );
    assert_eq!(
        registry.inspect_resource(&world, Health::TYPE_NAME)?,
        Some(vec![ReflectValue::Unsigned(500)])
    );
    for metadata in registry.types() {
        println!(
            "{:?} {}: {:?}",
            metadata.role, metadata.name, metadata.fields
        );
    }
    Ok(())
}
