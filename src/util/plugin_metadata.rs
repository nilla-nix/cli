use std::path::Path;
use tokio::process::Command as TokioCommand;

/// Command information with name and description
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
}

/// Cached metadata about a plugin
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub help_text: String,
    pub version: Option<String>,
    pub usage: Option<String>,
    pub commands: Vec<CommandInfo>,
    pub examples: Vec<String>,
    pub supports_completions: bool,
}

/// Execute a plugin command and return its output
async fn execute_plugin_command(plugin_path: &Path, args: &[&str]) -> Option<String> {
    let output = TokioCommand::new(plugin_path)
        .args(args)
        .output()
        .await
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

/// Parse plugin help text to extract usage, commands, examples, and completion support
fn parse_plugin_help(help_text: &str, plugin_name: &str) -> (Option<String>, Vec<CommandInfo>, Vec<String>, bool) {
    let mut usage = None;
    let mut commands = Vec::new();
    let mut examples = Vec::new();
    let supports_completions = help_text.contains("completion") || help_text.contains("completions");

    // Extract Usage line and transform it to use "nilla <plugin>" format
    for line in help_text.lines() {
        let line = line.trim();
        if line.starts_with("Usage:") || line.starts_with("USAGE:") {
            // Replace "nilla-<plugin>" with "nilla <plugin>" in the usage line
            // Use regex-like replacement: find "nilla-<word>" and replace with "nilla <word>"
            let transformed_usage = line
                .replace(&format!("nilla-{}", plugin_name), &format!("nilla {}", plugin_name));
            usage = Some(transformed_usage);
            break;
        }
    }

    // Extract commands from Commands: section with their descriptions
    if let Some(commands_start) = help_text.find("Commands:") {
        let commands_section = &help_text[commands_start..];
        if let Some(commands_end) = commands_section.find("\n\n") {
            let commands_text = &commands_section[..commands_end];
            commands = commands_text
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with("Options:") {
                        None
                    } else {
                        // Parse command name and description
                        // Format is typically: "  command_name    Description text"
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.is_empty() {
                            None
                        } else {
                            let name = parts[0].to_string();

                            // Skip "help" command as clap adds it automatically
                            if name == "help" {
                                return None;
                            }

                            // Description is everything after the command name
                            // Find where the description starts (after multiple spaces or after first word)
                            let desc_start = trimmed.find(parts[0]).unwrap_or(0) + parts[0].len();
                            let description = trimmed[desc_start..].trim_start().to_string();

                            Some(CommandInfo {
                                name,
                                description: if description.is_empty() { "".to_string() } else { description },
                            })
                        }
                    }
                })
                .collect();
        }
    }

    // Extract examples from Examples: section
    if let Some(examples_start) = help_text.find("Examples:") {
        let examples_section = &help_text[examples_start..];
        if let Some(examples_end) = examples_section.find("\n\n") {
            let examples_text = &examples_section[..examples_end];
            examples = examples_text
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with("Options:") || trimmed.starts_with("Commands:") {
                        None
                    } else if trimmed.starts_with("#") || trimmed.starts_with("$") {
                        // Skip comment lines and shell prompt lines
                        None
                    } else if trimmed.len() > 5 {
                        Some(trimmed.to_string())
                    } else {
                        None
                    }
                })
                .collect();
        }
    }

    (usage, commands, examples, supports_completions)
}

/// Clean version string by removing program name prefix
fn clean_version(version: &str) -> String {
    if let Some(last_space) = version.rfind(' ') {
        version[last_space + 1..].to_string()
    } else {
        version.to_string()
    }
}

/// Get all metadata for a plugin by executing commands once and caching results
pub async fn get_plugin_metadata(plugin_path: &Path) -> PluginMetadata {
    // Execute --help and --version in parallel
    let (help_result, version_result) = tokio::join!(
        execute_plugin_command(plugin_path, &["--help"]),
        execute_plugin_command(plugin_path, &["--version"])
    );

    let help_text = help_result.unwrap_or_default();
    let version = version_result
        .map(|v| clean_version(v.trim()))
        .filter(|v| !v.is_empty());

    // Extract plugin name from path (e.g., "nilla-home" -> "home")
    let plugin_name = plugin_path
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.strip_prefix("nilla-"))
        .unwrap_or("plugin");

    let (usage, commands, examples, supports_completions) = parse_plugin_help(&help_text, plugin_name);

    PluginMetadata {
        help_text,
        version,
        usage,
        commands,
        examples,
        supports_completions,
    }
}
