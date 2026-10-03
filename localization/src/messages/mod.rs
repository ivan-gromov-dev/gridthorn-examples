use std::path::Path;

use gridthorn::localization::{CatalogAsset, LocaleId, Localization, MessageId, MessageParameters};

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let languages = ["en-US", "ru", "ar-EG", "ja"];
    let catalogs = languages
        .iter()
        .map(|language| {
            Ok(CatalogAsset::load(
                LocaleId::new(language)?,
                root.join(format!("{language}.ftl")),
            )?)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let english = LocaleId::new("en-US")?;
    let mut localization = Localization::new(catalogs, english.clone(), vec![])?;
    let mut parameters = MessageParameters::new();
    parameters.set_text("name", "Иван")?;
    parameters.set_text("role", "worker")?;
    parameters.set_number("count", 5.0)?;
    parameters.set_number("amount", 12345.5)?;
    for language in languages {
        let locale = LocaleId::new(language)?;
        let fallback = if locale == english {
            vec![]
        } else {
            vec![english.clone()]
        };
        localization.select_locale(locale, fallback)?;
        println!("[{language}]");
        for id in ["welcome", "workers", "role", "balance", "fallback-only"] {
            let message = localization.format(&MessageId::new(id)?, &parameters)?;
            println!("{id} ({}): {}", message.locale, message.text);
        }
        println!("number: {}", localization.format_number(-12345.5, 2, true)?);
    }
    let snapshot = localization.format(&MessageId::new("welcome")?, &parameters)?;
    if CatalogAsset::from_source(LocaleId::new("ja")?, "welcome = {".to_owned()).is_ok() {
        return Err("malformed catalog unexpectedly accepted".into());
    }
    assert_eq!(
        localization.format(&MessageId::new("welcome")?, &parameters)?,
        snapshot
    );
    localization.replace_catalog(CatalogAsset::from_source(
        LocaleId::new("ja")?,
        "welcome = 更新: { $name }".to_owned(),
    )?)?;
    assert!(
        localization
            .format(&MessageId::new("welcome")?, &parameters)?
            .text
            .starts_with("更新")
    );
    assert!(
        localization
            .format(&MessageId::new("absent")?, &parameters)
            .is_err()
    );
    println!("Validated catalog replacement, fallback and missing-message diagnostics passed.");
    Ok(())
}

#[cfg(test)]
mod test;
