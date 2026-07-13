use log::info;

use crate::util::plugins;
use crate::util::plugin_metadata;

pub async fn plugins_list_cmd() -> anyhow::Result<()> {
    info!("Discovering plugins...");

    let discovered_plugins = plugins::discover_plugins()?;

    if discovered_plugins.is_empty() {
        println!("No plugins found in PATH.");
        return Ok(());
    }

    println!("Found {} plugin(s):\n", discovered_plugins.len());

    // Use a table format for better readability
    println!("{:<20} {:<50} {:<15} {:<10}", "NAME", "PATH", "VERSION", "COMPLETIONS");
    println!("{}", "-".repeat(95));

    for plugin in &discovered_plugins {
        let metadata = plugin_metadata::get_plugin_metadata(&plugin.path).await;

        let version = metadata.version.unwrap_or_else(|| "unknown".to_string());
        let completions_str = if metadata.supports_completions { "yes" } else { "no" };

        let path_str = plugin.path.display().to_string();
        // Truncate path if too long
        let path_display = if path_str.len() > 48 {
            format!("...{}", &path_str[path_str.len() - 45..])
        } else {
            path_str
        };

        println!("{:<20} {:<50} {:<15} {:<10}",
            plugin.name, path_display, version, completions_str);
    }

    Ok(())
}
