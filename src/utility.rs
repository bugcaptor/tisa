use std::{env, path::PathBuf};

pub fn to_absolute_path(relative_path: &str) -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let absolute_path = current_dir.join(relative_path);
    Ok(absolute_path)
}
