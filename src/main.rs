mod config;
use config::{Commands, Cli};
mod utils;
use utils::create_test_dir;

use std::env;
use std::path::{PathBuf, Path};
use std::fs;
use std::process::{Command};


use anyhow::{Result as AnyResult, Context, anyhow};
use clap::Parser;

fn main() -> AnyResult<()> {
    let cf = Cli::parse();

    let theme_dir : PathBuf = get_config_dir().context(
            concat!(
            "Cannot decide theme directory. (expects as least one of ",
            "EZA_THEME_DIR, XDG_DATA_HOME, HOME to be set)")
        )?;
    let theme_dir: &Path = theme_dir.as_path();
    let eza_dir : PathBuf = get_eza_dir().context(
            concat!(
            "Cannot decide eza directory. (expects as least one of ",
            "EZA_CONFIG_DIR, XDG_CONFIG_HOME, HOME to be set)")
        )?;
    let eza_dir: &Path = eza_dir.as_path();
    if ! eza_dir.exists() || ! eza_dir.is_dir() {
        return Err(anyhow!(
            format!(concat!(
                "Eza home does not exists at designated location ",
                "or is not a directory. ",
                "Refuse to create.\n",
                "Manually create it at {}"
            ), eza_dir.display())
        ));
    }
    fs::create_dir_all(theme_dir)?;

    match cf.cmd {
        Commands::List => {
            for entry in fs::read_dir(theme_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "yml" {
                            println!("{}", path.file_name().context(
                                    "Failed to get file name"
                            )?.display());
                        }
                    }
                }
            }
        },
        Commands::Switch {theme_name} => {
            println!("Getting {theme_name}");
            let mut theme_file = PathBuf::from(&theme_name);
            theme_file.set_extension("yml");
            let src = theme_dir.join(theme_file);
            let dst = eza_dir.join("theme.yml");
            fs::copy(src, dst)?;
            println!("Applied {theme_name}");
        },
        Commands::Add {theme_name, theme_path} => {
            let mut dst_theme_file = PathBuf::from(&theme_name);
            dst_theme_file.set_extension("yml");
            let dst = theme_dir.join(&dst_theme_file);
            fs::copy(theme_path, dst)?;
            println!("Added {}", dst_theme_file.display());
        },
        Commands::Preview {theme_name} => {
            let mut theme_file = PathBuf::from(&theme_name);
            theme_file.set_extension("yml");
            let src = theme_dir.join(&theme_file);

            if ! src.exists() {
                // ![TODO] improve error messages
                println!("{}", theme_file.display());
                return Err(anyhow!("No such file or directory"));
            }

            create_test_dir(theme_dir)?;

            let test_dir = theme_dir.join("test_dir");

            let dst = test_dir.join("theme.yml");
            fs::copy(src, dst)?;
            
            // replace the current process with eza
            let mut eza = Command::new("eza")
                .arg("--color")
                .arg("--icons")
                .arg("-lah")
                .arg("--group-directories-first")
                .arg(&format!("{}", test_dir.display()))
                .env("EZA_CONFIG_DIR", format!("{}", test_dir.display()))
                .spawn()?;
            let status = eza.wait()?;
            match status.code() {
                Some(code) => println!("Eza exit: {}", code),
                None => println!("Eza terminated by signal")
            };
        },
    }

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
