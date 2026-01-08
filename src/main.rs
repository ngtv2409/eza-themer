mod config;

use std::env;
use std::path::{PathBuf, Path};
use std::fs;

use anyhow::{Result as AnyResult, Context, anyhow};
use clap::Parser;

fn main() -> AnyResult<()> {
    let cf = config::Cli::parse();

    let theme_dir : PathBuf = get_config_dir().context(
            concat!(
            "Error: Cannot decide config directory. (expects as least one of ",
            "EZA_THEME_DIR, XDG_DATA_HOME, HOME to be set)")
        )?;
    let theme_dir: &Path = theme_dir.as_path();
    let eza_dir : PathBuf = get_eza_dir().context(
            concat!(
            "Error: Cannot decide eza directory. (expects as least one of ",
            "EZA_CONFIG_DIR, XDG_CONFIG_HOME, HOME to be set)")
        )?;
    let eza_dir: &Path = theme_dir.as_path();
    if ! eza_dir.exists() {
        return Err(anyhow!(
            concat!(
                "Eza home does not exists at designated location ",
                "or is not a directory. ",
                "Refuse to create."
            )
        ));
    }
    fs::create_dir_all(theme_dir)?;

    println!("{theme_dir:?}");
    
    Ok(())
}

fn get_config_dir() -> Option<PathBuf> {
    env::var("EZA_THEME_DIR").ok().map(PathBuf::from)
        .or_else(|| {
            env::var("XDG_DATA_HOME")
                .ok()
                .map(|p| PathBuf::from(p).join("eza-themes"))
        })
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(|p| PathBuf::from(p).join(".local").join("share").join("eza-themes"))
        })
}

fn get_eza_dir() -> Option<PathBuf> {
    env::var("EZA_CONFIG_DIR").ok().map(PathBuf::from)
        .or_else(|| {
            env::var("XDG_CONFIG_HOME")
                .ok()
                .map(|p| PathBuf::from(p).join("eza"))
        })
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(|p| PathBuf::from(p).join(".config").join("eza"))
        })
}
