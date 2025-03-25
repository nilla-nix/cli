use std::path::PathBuf;

pub fn search_up_for_file(start: &PathBuf, file: &str) -> Option<PathBuf> {
    let mut current = start.clone();
    loop {
        let candidate = current.join(file);
        if candidate.is_file() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}

pub fn search_up_for_dir(start: &PathBuf, dir: &str) -> Option<PathBuf> {
    let mut current = start.clone();
    loop {
        let candidate = current.join(dir);
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}
