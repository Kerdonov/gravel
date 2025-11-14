use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Recursively crawl through the directory given and aggregate all file handles to a `HashMap` with
/// their respective (relative) paths as keys.
#[must_use]
pub fn crawl_fs(root: &PathBuf) -> HashSet<PathBuf> {
    crawl_fs_rec(root, root)
}

/// Helper function: Recursively crawl through the directory given and aggregate all file handles to a `HashMap` with
/// their respective (relative) paths as keys.
fn crawl_fs_rec(root: &PathBuf, path: &PathBuf) -> HashSet<PathBuf> {
    let mut subdirs = Vec::with_capacity(100);
    let mut files = HashSet::with_capacity(1024);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if entry.path().is_dir() {
                            subdirs.push(entry.path());
                        } else {
                            let path = entry.path();
                            if let Ok(path_no_prefix) = path.strip_prefix(root) {
                                files.insert(path_no_prefix.into());
                            }
                        }
                    }
                    Err(_e) => {}
                }
            }
        }
        Err(_e) => {}
    }

    let others: HashSet<PathBuf> = subdirs
        .into_iter()
        .flat_map(|s| crawl_fs_rec(root, &s))
        .collect();

    files.extend(others);
    files
}
