use gridthorn::{Reflect, SceneDocument, SceneValueError};

use super::model::Player;

pub(super) fn rename_health(mut document: SceneDocument) -> Result<SceneDocument, SceneValueError> {
    for entity in &mut document.entities {
        for record in &mut entity.components {
            if record.type_name == Player::TYPE_NAME {
                let health = record
                    .fields
                    .remove("hp")
                    .ok_or_else(|| SceneValueError("legacy player lacks hp".into()))?;
                record.fields.insert("health".into(), health);
            }
        }
    }
    document.schema_version = 1;
    Ok(document)
}
