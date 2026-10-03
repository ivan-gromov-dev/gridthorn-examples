use super::region;
use gridthorn::{
    AnimationClip, AnimationPlayback, AnimationPlayer, AssetId, AssetReloader, AssetStore,
    SpriteRegion, TextureAsset,
};
use std::time::{Duration, Instant};

pub(crate) struct Assets {
    reloader: AssetReloader,
    atlas: AssetId,
    expansion: AssetId,
    skin: AssetId,
    animation: AnimationPlayer,
    next_scan: Instant,
    reloads: u64,
    error: Option<String>,
}

impl Assets {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let mut store = AssetStore::new(root);
        let atlas = AssetId::new("harbor-atlas.png")?;
        let manifest = AssetId::new("skin.txt")?;
        store.load_texture(atlas.clone())?;
        let expansion = AssetId::new("harbor-expansion.png")?;
        let skin = AssetId::new("harbor-ui.png")?;
        store.load_texture(expansion.clone())?;
        store.load_texture(skin.clone())?;
        store.load_source(manifest.clone())?;
        store.set_dependencies(&manifest, &[atlas.clone(), expansion.clone(), skin.clone()])?;
        let animation = AnimationPlayer::new(AnimationClip::new(
            vec![region(10), region(11)],
            Duration::from_millis(160),
            AnimationPlayback::Loop,
        )?);
        Ok(Self {
            reloader: AssetReloader::new(store)?,
            atlas,
            expansion,
            skin,
            animation,
            next_scan: Instant::now(),
            reloads: 0,
            error: None,
        })
    }

    pub(super) fn poll(&mut self) {
        match self.reloader.poll() {
            Ok(Some(changed)) => {
                if !changed.is_empty() {
                    self.reloads += 1;
                    self.error = None;
                }
            }
            Err(error) => {
                self.error = Some(error.to_string());
            }
            Ok(None) => {}
        }
        if Instant::now() >= self.next_scan && !self.reloader.is_pending() {
            if let Err(error) = self.reloader.request_reload() {
                self.error = Some(error.to_string());
            }
            self.next_scan = Instant::now() + Duration::from_millis(250);
        }
    }
}

impl Assets {
    pub(super) fn snapshot(
        &mut self,
        elapsed: Duration,
    ) -> (
        TextureAsset,
        TextureAsset,
        TextureAsset,
        SpriteRegion,
        u64,
        Option<String>,
    ) {
        self.animation.advance(elapsed);
        (
            self.reloader
                .assets()
                .texture(&self.atlas)
                .expect("last good atlas")
                .clone(),
            self.reloader
                .assets()
                .texture(&self.expansion)
                .expect("expansion atlas")
                .clone(),
            self.reloader
                .assets()
                .texture(&self.skin)
                .expect("UI atlas")
                .clone(),
            self.animation.region(),
            self.reloads,
            self.error.clone(),
        )
    }
    pub(super) fn shutdown(&mut self) {
        if let Err(error) = self.reloader.shutdown() {
            eprintln!("Asset worker shutdown: {error}");
        }
    }
}
