use std::path::{Path, PathBuf};

/// Locate one asset root shared by presentation and audio.
pub(crate) fn directory() -> std::io::Result<PathBuf> {
    let executable = std::env::current_exe()?;
    Ok(directory_for_executable(
        &executable,
        Path::new(env!("CARGO_MANIFEST_DIR")),
    ))
}

pub(super) fn directory_for_executable(executable: &Path, source: &Path) -> PathBuf {
    if let Some(parent) = executable.parent() {
        let packaged = parent.join("assets");
        if packaged.is_dir() {
            return packaged;
        }
    }
    source.join("assets")
}
