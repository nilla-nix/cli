use log::trace;
use std::path::PathBuf;

pub fn search_up_for_file<P>(start: P, file: &str) -> Option<PathBuf>
where
    P: Into<PathBuf>,
{
    let mut current: PathBuf = start.into();
    trace!("Searching up for file '{file}' starting at {current:?}");
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

pub fn search_up_for_dir<P>(start: P, dir: &str) -> Option<PathBuf>
where
    P: Into<PathBuf>,
{
    let mut current: PathBuf = start.into();
    trace!("Searching up for directory '{dir}' starting at {current:?}");
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
