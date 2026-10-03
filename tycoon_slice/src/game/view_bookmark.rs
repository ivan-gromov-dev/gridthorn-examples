use super::session::{Session, scene};
use gridthorn::{
    FieldMetadata, Reflect, ReflectValue, SceneData, SceneDocument, SceneRegistry, SceneValueError,
    ScheduleBuilder, ValueKind,
};

/// Non-authoritative camera data uses the previous milestone's scalar scene contract.
struct Bookmark {
    camera: [f32; 2],
    height: f32,
    square: bool,
}

impl Reflect for Bookmark {
    const TYPE_NAME: &'static str = "harbor.camera";
    const FIELDS: &'static [FieldMetadata] = &[
        FieldMetadata {
            name: "x",
            kind: ValueKind::Float,
        },
        FieldMetadata {
            name: "y",
            kind: ValueKind::Float,
        },
        FieldMetadata {
            name: "height",
            kind: ValueKind::Float,
        },
        FieldMetadata {
            name: "square",
            kind: ValueKind::Bool,
        },
    ];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![
            ReflectValue::Float(f64::from(self.camera[0])),
            ReflectValue::Float(f64::from(self.camera[1])),
            ReflectValue::Float(f64::from(self.height)),
            ReflectValue::Bool(self.square),
        ]
    }
}

impl SceneData for Bookmark {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Validated bounded camera scalars are converted to renderer f32 units"
    )]
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError> {
        match values {
            [
                ReflectValue::Float(x),
                ReflectValue::Float(y),
                ReflectValue::Float(height),
                ReflectValue::Bool(square),
            ] if x.is_finite()
                && y.is_finite()
                && x.abs() <= 600.0
                && y.abs() <= 600.0
                && (400.0..=1200.0).contains(height) =>
            {
                Ok(Self {
                    camera: [*x as f32, *y as f32],
                    height: *height as f32,
                    square: *square,
                })
            }
            _ => Err(SceneValueError(
                "camera bookmark is outside supported bounds".into(),
            )),
        }
    }
}

pub fn capture(session: &Session) -> Result<String, Box<dyn std::error::Error>> {
    let mut registry = SceneRegistry::default();
    registry.register_resource::<Bookmark>()?;
    let mut schedules = ScheduleBuilder::new().build();
    let mut world = schedules.world();
    world.insert_resource(Bookmark {
        camera: session.camera,
        height: session.height,
        square: session.square,
    });
    Ok(registry
        .capture(&mut world, &scene("camera-bookmark"))?
        .to_toml()?)
}

pub fn restore(session: &mut Session, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = SceneRegistry::default();
    registry.register_resource::<Bookmark>()?;
    let document = SceneDocument::from_toml(source)?;
    let prepared = registry.prepare(&document)?;
    let mut schedules = ScheduleBuilder::new().build();
    let mut world = schedules.world();
    let _ = prepared.commit(&mut world);
    let (camera, height, square) = world
        .read_resource(|b: &Bookmark| (b.camera, b.height, b.square))
        .ok_or("camera bookmark missing resource")?;
    session.camera = camera;
    session.height = height;
    session.square = square;
    Ok(())
}
