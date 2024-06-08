use std::io;
use std::{fs, path::Path};

use glob::Pattern;

pub fn find_files_by_pattern<P: AsRef<Path>>(dir: P, pattern: &str) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let pattern = Pattern::new(pattern).unwrap();

    fn visit_dirs(dir: &Path, pattern: &Pattern, results: &mut Vec<String>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, pattern, results)?;
                } else {
                    if let Some(path_str) = path.to_str() {
                        if pattern.matches(path_str) {
                            results.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    visit_dirs(dir.as_ref(), &pattern, &mut results)?;

    Ok(results)
}
