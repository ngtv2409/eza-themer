use std::env;
use std::path::{PathBuf};

use anyhow::{Result as AnyResult, Context};

fn main() -> AnyResult<()> {
    let theme_dir = get_config_dir().context(
            concat!(
            "Error: Cannot decide config directory. (expects as least one of ",
            "EZA_THEME_DIR, XDG_CONFIG_PATH, HOME to be set)")
        )?;

    println!("{theme_dir:?}");
    
    Ok(())
}

fn get_config_dir() -> Option<PathBuf> {
    env::var("EZA_THEME_DIR").ok().map(PathBuf::from)
        .or_else(|| {
            env::var("XDG_CONFIG_PATH")
                .ok()
                .map(|p| PathBuf::from(p).join("eza-themes"))
        })
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(|p| PathBuf::from(p).join(".config").join("eza-themes"))
        })
}
