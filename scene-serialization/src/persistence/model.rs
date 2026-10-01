use gridthorn::{FieldMetadata, Reflect, ReflectValue, SceneData, SceneValueError, ValueKind};

pub(super) struct Player {
    pub(super) name: String,
    pub(super) health: u64,
    pub(super) cached_label: String,
}

impl Player {
    pub(super) fn new(name: String, health: u64) -> Result<Self, SceneValueError> {
        if name.trim().is_empty() || health == 0 {
            return Err(SceneValueError(
                "player needs a name and positive health".into(),
            ));
        }
        let cached_label = format!("{name}: {health}");
        Ok(Self {
            name,
            health,
            cached_label,
        })
    }
}

impl Reflect for Player {
    const TYPE_NAME: &'static str = "example.player";
    const FIELDS: &'static [FieldMetadata] = &[
        FieldMetadata {
            name: "name",
            kind: ValueKind::Text,
        },
        FieldMetadata {
            name: "health",
            kind: ValueKind::Unsigned,
        },
    ];

    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![
            ReflectValue::Text(self.name.clone()),
            ReflectValue::Unsigned(self.health),
        ]
    }
}

impl SceneData for Player {
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError> {
        match values {
            [ReflectValue::Text(name), ReflectValue::Unsigned(health)] => {
                Self::new(name.clone(), *health)
            }
            _ => Err(SceneValueError("invalid player fields".into())),
        }
    }
}

pub(super) struct Score(pub(super) u64);

impl Reflect for Score {
    const TYPE_NAME: &'static str = "example.score";
    const FIELDS: &'static [FieldMetadata] = &[FieldMetadata {
        name: "score",
        kind: ValueKind::Unsigned,
    }];

    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Unsigned(self.0)]
    }
}

impl SceneData for Score {
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError> {
        match values {
            [ReflectValue::Unsigned(score)] => Ok(Self(*score)),
            _ => Err(SceneValueError("invalid score field".into())),
        }
    }
}
