use super::super::{Preferences, errors::PreferenceError};
use gridthorn::{GraphicsBackend, GraphicsDeviceKey, GraphicsSelection};
use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);
fn temporary_preferences() -> Preferences {
    Preferences {
        path: std::env::temp_dir().join(format!(
            "gridthorn-split-api-{}-{}.txt",
            std::process::id(),
            NEXT_FILE.fetch_add(1, Ordering::Relaxed)
        )),
    }
}
#[test]
fn roundtrips_independent_preferences_and_clears_automatic_selection() {
    let preferences = temporary_preferences();
    assert_eq!(preferences.load().unwrap(), GraphicsSelection::default());
    for selection in [
        GraphicsSelection {
            device: Some(GraphicsDeviceKey {
                vendor: 1,
                device: 2,
                name: "Тест GPU\nline".into(),
            }),
            api: None,
        },
        GraphicsSelection {
            device: None,
            api: Some(GraphicsBackend::Vulkan),
        },
        GraphicsSelection {
            device: Some(GraphicsDeviceKey {
                vendor: 1,
                device: 2,
                name: "GPU".into(),
            }),
            api: Some(GraphicsBackend::Direct3D12),
        },
    ] {
        preferences.save(&selection).unwrap();
        assert_eq!(preferences.load().unwrap(), selection);
    }
    preferences.save(&GraphicsSelection::default()).unwrap();
    assert_eq!(preferences.load().unwrap(), GraphicsSelection::default());
}
#[test]
fn migrates_the_previous_combined_preference_and_reports_corruption() {
    let preferences = temporary_preferences();
    fs::write(
        &preferences.path,
        "Vulkan\n4318\n9352\nNVIDIA GeForce RTX 3070",
    )
    .unwrap();
    let selection = preferences.load().unwrap();
    assert_eq!(selection.api, Some(GraphicsBackend::Vulkan));
    assert_eq!(selection.device.unwrap().device, 9352);
    fs::write(&preferences.path, "broken preference").unwrap();
    assert!(matches!(preferences.load(), Err(PreferenceError::Invalid)));
    fs::remove_file(&preferences.path).unwrap();
    let unreachable = Preferences {
        path: preferences.path.join("missing").join("preferences.txt"),
    };
    assert!(matches!(
        unreachable.save(&GraphicsSelection {
            device: None,
            api: Some(GraphicsBackend::Vulkan)
        }),
        Err(PreferenceError::Io(_))
    ));
}
