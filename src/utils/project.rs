use std::path::{Path, PathBuf};

pub fn find_project_root(start: &Path) -> PathBuf {
    let mut current = Some(start);

    while let Some(dir) = current {
        if dir.join("Cargo.toml").exists() {
            return dir.to_path_buf();
        }
        current = dir.parent();
    }

    start.to_path_buf()
}
