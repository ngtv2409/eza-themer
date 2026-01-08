use std::env;
use std::path::{PathBuf, Path};
use std::fs;

use anyhow::{Result as AnyResult, Context};

fn main() -> AnyResult<()> {
    let theme_dir : PathBuf = get_config_dir().context(
            concat!(
            "Error: Cannot decide config directory. (expects as least one of ",
            "EZA_THEME_DIR, XDG_DATA_HOME, HOME to be set)")
        )?;
    let theme_dir: &Path = theme_dir.as_path();
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
