//! Initialize PAII configuration

use colored::*;
use eyre::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::config::Config;

pub fn run(path: Option<PathBuf>, force: bool) -> Result<()> {
    let paii_dir = path.unwrap_or_else(Config::paii_dir);

    println!("{} Initializing PAII in {}", "→".blue(), paii_dir.display());

    // Check if already initialized
    let config_file = paii_dir.join("paii.toml");
    if config_file.exists() && !force {
        println!("  {} PAII already initialized at {}", "✓".green(), paii_dir.display());
        println!("  Use {} to reinitialize", "--force".cyan());
        return Ok(());
    }

    // Create directory structure
    let dirs = ["plugins", "history", "registries"];
    for dir in &dirs {
        let dir_path = paii_dir.join(dir);
        fs::create_dir_all(&dir_path).context(format!("Failed to create {}", dir))?;
        println!("  {} Created {}/", "✓".green(), dir);
    }

    // Create history subdirectories
    let history_dirs = ["sessions", "decisions", "learnings", "errors"];
    for dir in &history_dirs {
        let dir_path = paii_dir.join("history").join(dir);
        fs::create_dir_all(&dir_path).context(format!("Failed to create history/{}", dir))?;
    }
    println!("  {} Created history subdirectories", "✓".green());

    // Generate default config
    let config = Config::default();
    let toml_str = toml::to_string_pretty(&config).context("Failed to serialize config")?;
    fs::write(&config_file, toml_str).context("Failed to write paii.toml")?;
    println!("  {} Created paii.toml", "✓".green());

    println!();
    println!("{} PAII initialized!", "✓".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. Run {} to verify setup", "paii doctor".cyan());
    println!("  2. Run {} to fetch plugins", "paii registry update".cyan());
    println!("  3. Run {} to find plugins", "paii registry search <query>".cyan());

    Ok(())
}
