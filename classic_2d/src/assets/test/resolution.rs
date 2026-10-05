use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::resolution::directory_for_executable;

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct AssetFixture(PathBuf);

impl AssetFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "classic-assets-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
}

impl Drop for AssetFixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn source_assets_support_development_without_a_packaged_directory() {
    let fixture = AssetFixture::new();
    let source = fixture.0.join("source");
    assert_eq!(
        directory_for_executable(&fixture.0.join("game.exe"), &source),
        source.join("assets")
    );
}

#[test]
fn incomplete_packaged_assets_do_not_fall_back_to_source() {
    let fixture = AssetFixture::new();
    let packaged = fixture.0.join("assets");
    let source = fixture.0.join("source");
    std::fs::create_dir(&packaged).unwrap();
    std::fs::create_dir_all(source.join("assets")).unwrap();
    std::fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/atlas.ppm"),
        source.join("assets/atlas.ppm"),
    )
    .unwrap();
    let selected = directory_for_executable(&fixture.0.join("game.exe"), &source);
    assert_eq!(selected, packaged);
    assert!(gridthorn::TextureAsset::load(selected.join("atlas.ppm")).is_err());
}
