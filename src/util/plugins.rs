use std::collections::HashSet;
use std::path::{Path, PathBuf};
use anyhow::Result;
use log::debug;

/// Information about a discovered plugin
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub path: PathBuf,
}

/// Discover all available plugins in PATH
///
/// Searches PATH for executables matching the pattern `nilla-*` and returns
/// information about each discovered plugin.
pub fn discover_plugins() -> Result<Vec<PluginInfo>> {
    let path_var = std::env::var("PATH")?;
    // PATH separator is ':' on Unix and ';' on Windows
    let path_dirs: Vec<&str> = if cfg!(windows) {
        path_var.split(';').collect()
    } else {
        path_var.split(':').collect()
    };

    let mut plugins = Vec::new();
    let mut seen_names = HashSet::new();

    for dir in path_dirs {
        let dir_path = Path::new(dir);
        if !dir_path.is_dir() {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(dir_path) else {
            continue;
        };

        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };

            let path = entry.path();

            // Check if it's an executable file starting with "nilla-"
            if path.is_file() {
                // Extract file_name as owned String before moving path
                let file_name_opt = path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string());

                if let Some(file_name) = file_name_opt {
                    if file_name.starts_with("nilla-") && file_name != "nilla" {
                        // Check if it's executable (on Unix-like systems)
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let metadata = match std::fs::metadata(&path) {
                                Ok(m) => m,
                                Err(_) => continue,
                            };
                            if metadata.permissions().mode() & 0o111 == 0 {
                                continue; // Not executable
                            }
                        }

                        let plugin_name = file_name.strip_prefix("nilla-").unwrap().to_string();

                        // Avoid duplicates (same plugin name in different PATH directories)
                        if seen_names.insert(plugin_name.clone()) {
                            // Store original path for debug before moving
                            let path_for_debug = path.clone();
                            let canonicalized_path = path.canonicalize().unwrap_or_else(|_| path);
                            plugins.push(PluginInfo {
                                name: plugin_name,
                                path: canonicalized_path,
                            });
                            debug!("Discovered plugin: {} at {:?}", file_name, path_for_debug);
                        }
                    }
                }
            }
        }
    }

    // Sort by name for consistent output
    plugins.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(plugins)
}

/// Get plugin information by name
pub fn find_plugin(name: &str) -> Result<Option<PluginInfo>> {
    let plugin_name = format!("nilla-{}", name);

    match which::which(&plugin_name) {
        Ok(path) => Ok(Some(PluginInfo {
            name: name.to_string(),
            path: path.canonicalize().unwrap_or(path),
        })),
        Err(_) => Ok(None),
    }
}
