use gridthorn::{
    SceneDocument, SceneId, SceneMigrations, SceneRegistry, SceneScalar, ScheduleBuilder,
};

use super::{
    migration::rename_health,
    model::{Player, Score},
};

/// Exercise scene round-trip, off-world validation, and an explicit legacy migration.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = SceneRegistry::default();
    registry.register_component::<Player>()?;
    registry.register_resource::<Score>()?;
    let scene = SceneId::new("level.one")?;
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    let original = world.spawn_in_scene(scene.clone(), Player::new("Ada".into(), 100)?);
    world.insert_resource(Score(42));
    let document = registry.capture(&mut world, &scene)?;
    let source = document.to_toml()?;
    assert!(!source.contains("cached_label"));
    let restored = registry
        .prepare(&SceneDocument::from_toml(&source)?)?
        .commit(&mut world);
    assert_eq!(
        world.read_component(original, |player: &Player| player.health),
        None
    );
    assert_eq!(
        world.read_component(restored.entities[0], |player: &Player| player
            .cached_label
            .clone()),
        Some("Ada: 100".into())
    );
    assert_eq!(world.read_resource(|score: &Score| score.0), Some(42));

    let before = registry.capture(&mut world, &scene)?;
    let mut invalid = before.clone();
    invalid.entities[0].components[0]
        .fields
        .insert("health".into(), SceneScalar::Unsigned("0".into()));
    let error = registry
        .prepare(&invalid)
        .err()
        .ok_or("invalid health should fail")?;
    assert_eq!(registry.capture(&mut world, &scene)?, before);
    println!("Rejected invalid edit without changing world: {error}");

    let legacy_source = source
        .replace("schema_version = 1", "schema_version = 0")
        .replace(".health]", ".hp]");
    let legacy = SceneDocument::from_toml(&legacy_source)?;
    let mut migrations = SceneMigrations::default();
    migrations.register(0, rename_health)?;
    let upgraded = migrations.upgrade(legacy)?;
    let loaded = registry.prepare(&upgraded)?.commit(&mut world);
    assert_eq!(
        world.read_component(loaded.entities[0], |player: &Player| player.health),
        Some(100)
    );
    println!("Round-trip and legacy migration passed. Schema 1 scene:\n{source}");
    Ok(())
}
