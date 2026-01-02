use colored::*;
use eyre::{Context, Result};
use std::fs;

use crate::cli::RegistryAction;
use crate::config::Config;

pub fn run(action: RegistryAction, config: &Config) -> Result<()> {
    match action {
        RegistryAction::List => list(config),
        RegistryAction::Add { name, url } => add(&name, &url, config),
        RegistryAction::Remove { name } => remove(&name, config),
        RegistryAction::Update { name } => update(name.as_deref(), config),
    }
}

fn list(config: &Config) -> Result<()> {
    println!("{}", "Configured registries:".bold());
    println!();

    if config.registries.is_empty() {
        println!("  {}", "(none)".dimmed());
        return Ok(());
    }

    let registries_dir = Config::expand_path(&config.paths.registries);

    for (name, url) in &config.registries {
        // Check if we have a cached listing
        let cache_file = registries_dir.join(format!("{}.toml", name));
        let status = if cache_file.exists() {
            "✓".green().to_string()
        } else {
            "○".dimmed().to_string()
        };
        println!("  {} {}: {}", status, name.cyan(), url.dimmed());
    }

    Ok(())
}

fn add(name: &str, url: &str, config: &Config) -> Result<()> {
    println!("{} Adding registry: {} → {}", "→".blue(), name.cyan(), url.dimmed());

    // Check if already exists
    if config.registries.contains_key(name) {
        eyre::bail!("Registry '{}' already exists. Remove it first.", name);
    }

    // Update config
    let mut new_config = config.clone();
    new_config.registries.insert(name.to_string(), url.to_string());

    // Save config
    let config_path = Config::paii_dir().join("paii.toml");
    fs::create_dir_all(config_path.parent().unwrap())?;

    let toml_str = toml::to_string_pretty(&new_config).context("Failed to serialize config")?;
    fs::write(&config_path, toml_str).context("Failed to write config file")?;

    println!("  {} Added registry: {}", "✓".green(), name);

    Ok(())
}

fn remove(name: &str, config: &Config) -> Result<()> {
    println!("{} Removing registry: {}", "→".blue(), name.cyan());

    // Check if exists
    if !config.registries.contains_key(name) {
        eyre::bail!("Registry '{}' not found", name);
    }

    // Update config
    let mut new_config = config.clone();
    new_config.registries.remove(name);

    // Save config
    let config_path = Config::paii_dir().join("paii.toml");
    let toml_str = toml::to_string_pretty(&new_config).context("Failed to serialize config")?;
    fs::write(&config_path, toml_str).context("Failed to write config file")?;

    // Remove cached listing if exists
    let registries_dir = Config::expand_path(&config.paths.registries);
    let cache_file = registries_dir.join(format!("{}.toml", name));
    if cache_file.exists() {
        fs::remove_file(&cache_file).ok();
    }

    println!("  {} Removed registry: {}", "✓".green(), name);

    Ok(())
}

fn update(name: Option<&str>, config: &Config) -> Result<()> {
    let registries_dir = Config::expand_path(&config.paths.registries);
    fs::create_dir_all(&registries_dir).context("Failed to create registries directory")?;

    let registries_to_update: Vec<(String, String)> = match name {
        Some(n) => {
            if let Some(url) = config.registries.get(n) {
                vec![(n.to_string(), url.clone())]
            } else {
                eyre::bail!("Registry '{}' not found", n);
            }
        }
        None => config.registries.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
    };

    if registries_to_update.is_empty() {
        println!("  {}", "No registries configured".dimmed());
        return Ok(());
    }

    for (reg_name, url) in &registries_to_update {
        println!("{} Updating registry: {}", "→".blue(), reg_name.cyan());

        // For now, we can't actually fetch URLs without an HTTP client
        // Just create a placeholder or check if it's a local file
        if url.starts_with("file://") || url.starts_with('/') || url.starts_with('.') {
            // Local file registry
            let path = url.trim_start_matches("file://");
            if std::path::Path::new(path).exists() {
                let content = fs::read_to_string(path).context("Failed to read local registry")?;
                let cache_file = registries_dir.join(format!("{}.toml", reg_name));
                fs::write(&cache_file, content).context("Failed to cache registry")?;
                println!("  {} Cached from local file", "✓".green());
            } else {
                println!("  {} Local file not found: {}", "✗".red(), path);
            }
        } else {
            // Remote URL - would need HTTP client (reqwest, ureq, etc.)
            println!("  {} Remote fetching not implemented yet", "⚠".yellow());
            println!("    URL: {}", url.dimmed());
            println!("    {} Add 'reqwest' or 'ureq' crate for HTTP support", "ℹ".blue());
        }
    }

    Ok(())
}
