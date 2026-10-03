use super::session::Session;
use gridthorn::{FieldMetadata, Reflect, ReflectValue, ReflectionRegistry, ValueKind, WorldAccess};

pub struct SceneBanner;

pub struct Metrics {
    pub coins: u64,
    pub shipped: u64,
    pub tick: u64,
}

impl Reflect for Metrics {
    const TYPE_NAME: &'static str = "harbor.metrics";
    const FIELDS: &'static [FieldMetadata] = &[
        FieldMetadata {
            name: "coins",
            kind: ValueKind::Unsigned,
        },
        FieldMetadata {
            name: "shipped",
            kind: ValueKind::Unsigned,
        },
        FieldMetadata {
            name: "tick",
            kind: ValueKind::Unsigned,
        },
    ];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![
            ReflectValue::Unsigned(self.coins),
            ReflectValue::Unsigned(self.shipped),
            ReflectValue::Unsigned(self.tick),
        ]
    }
}

pub fn publish(world: &mut WorldAccess<'_>) {
    let metrics = world
        .read_resource(|s: &Session| Metrics {
            coins: s.data.coins,
            shipped: s.data.shipped,
            tick: s.ticks,
        })
        .expect("session");
    world.insert_resource(metrics);
    let mut registry = ReflectionRegistry::default();
    registry
        .register_resource::<Metrics>()
        .expect("metrics metadata");
    let values = registry
        .inspect_resource(world, Metrics::TYPE_NAME)
        .expect("metrics inspection")
        .expect("metrics");
    world.insert_resource(Inspector(values));
}

pub struct Inspector(pub Vec<ReflectValue>);
