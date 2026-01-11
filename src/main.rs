mod config;
use config::{Cli, Commands};
mod utils;
use utils::{create_test_dir, merge_yaml_files};

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result as AnyResult, anyhow};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};

fn main() -> AnyResult<()> {
    let cf = Cli::parse();

    let theme_dir: PathBuf = get_config_dir().context(concat!(
        "Cannot decide theme directory. (expects as least one of ",
        "EZA_THEME_DIR, XDG_DATA_HOME, HOME to be set)"
    ))?;
    let theme_dir: &Path = theme_dir.as_path();
    let eza_dir: PathBuf = get_eza_dir().context(concat!(
        "Cannot decide eza directory. (expects as least one of ",
        "EZA_CONFIG_DIR, XDG_CONFIG_HOME, HOME to be set)"
    ))?;
    let eza_dir: &Path = eza_dir.as_path();
    if !eza_dir.exists() || !eza_dir.is_dir() {
        return Err(anyhow!(format!(
            concat!(
                "Eza home does not exists at designated location ",
                "or is not a directory. ",
                "Refuse to create.\n",
                "Manually create it at {}"
            ),
            eza_dir.display()
        )));
    }
    fs::create_dir_all(theme_dir).context(format!(
        "Failed to create theme directory at {}",
        theme_dir.display()
    ))?;

    // .files are ezt internal files, to distinguish from themes
    let overlay_path = theme_dir.join(".overlay.yml");

    match cf.cmd {
        Commands::List => {
            for entry in fs::read_dir(theme_dir)
                .context(format!("Failed to read directory {}", theme_dir.display()))?
            {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "yml" {
                            println!(
                                "{}",
                                path.file_name()
                                    .context("Failed to get file name")?
                                    .display()
                            );
                        }
                    }
                }
            }
        }
        Commands::Switch {
            theme_name,
            interactive,
        } => {
            let theme_name = if interactive {
                let mut themes: Vec<String> = Vec::new();
                for entry in fs::read_dir(theme_dir)
                    .context(format!("Failed to read directory {}", theme_dir.display()))?
                {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "yml" {
                                themes.push(
                                    path.file_name()
                                        .ok_or_else(|| anyhow!("Failed to get filename"))?
                                        .to_str()
                                        .ok_or_else(|| anyhow!("Filename is not valid UTF-8"))?
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&themes)
                    .default(0)
                    .interact()
                    .unwrap();
                themes[selection].clone()
            } else {
                // if not interact, theme name is required
                theme_name.unwrap()
            };
            println!("Getting {theme_name}");
            let mut theme_file = PathBuf::from(&theme_name);
            theme_file.set_extension("yml");
            let src = theme_dir.join(&theme_file);
            let dst = eza_dir.join("theme.yml");
            if overlay_path.exists() {
                merge_yaml_files(&src, &overlay_path, &dst)?;
            } else {
                fs::copy(&src, &dst).context(format!(
                    "Failed to copy files {} -> {}",
                    src.display(),
                    dst.display()
                ))?;
            }
            println!("Applied {theme_name}");
        }
        Commands::Add {
            theme_name,
            theme_path,
        } => {
            let mut dst_theme_file = PathBuf::from(&theme_name);
            dst_theme_file.set_extension("yml");
            let dst = theme_dir.join(&dst_theme_file);
            fs::copy(&theme_path, &dst).context(format!(
                "Failed to copy files {} -> {}",
                theme_path,
                dst.display()
            ))?;
            println!("Added {}", dst_theme_file.display());
        }
        Commands::Preview {
            theme_name,
            interactive,
        } => {
            let theme_name = if interactive {
                let mut themes: Vec<String> = Vec::new();
                for entry in fs::read_dir(theme_dir)
                    .context(format!("Failed to read directory {}", theme_dir.display()))?
                {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "yml" {
                                themes.push(
                                    path.file_name()
                                        .ok_or_else(|| anyhow!("Failed to get filename"))?
                                        .to_str()
                                        .ok_or_else(|| anyhow!("Filename is not valid UTF-8"))?
                                        .to_string(),
                                );
                            }
                        }
                    }
                }
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&themes)
                    .default(0)
                    .interact()
                    .unwrap();
                themes[selection].clone()
            } else {
                // if not interact, theme name is required
                theme_name.unwrap()
            };
            let mut theme_file = PathBuf::from(&theme_name);
            theme_file.set_extension("yml");
            let src = theme_dir.join(&theme_file);

            if !src.exists() {
                // ![TODO] improve error messages
                println!("{}", theme_file.display());
                return Err(anyhow!("No such file or directory"));
            }

            create_test_dir(theme_dir).context("Failed to create test dir")?;

            let test_dir = theme_dir.join("test_dir");

            let dst = test_dir.join("theme.yml");
            fs::copy(&src, &dst).context(format!(
                "Failed to copy files {} -> {}",
                src.display(),
                dst.display()
            ))?;

            // replace the current process with eza
            let mut eza = Command::new("eza")
                .arg("--color")
                .arg("--icons")
                .arg("-lah")
                .arg("--group-directories-first")
                .arg(&format!("{}", test_dir.display()))
                .env("EZA_CONFIG_DIR", format!("{}", test_dir.display()))
                .spawn()
                .context("Failed to spawn eza")?;
            let status = eza.wait()?;
            match status.code() {
                Some(code) => println!("Eza exit: {}", code),
                None => println!("Eza terminated by signal"),
            };
        }
    }

    Ok(())
}

fn get_config_dir() -> Option<PathBuf> {
    env::var("EZA_THEME_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            env::var("XDG_DATA_HOME")
                .ok()
                .map(|p| PathBuf::from(p).join("eza-themes"))
        })
        .or_else(|| {
            env::var("HOME").ok().map(|p| {
                PathBuf::from(p)
                    .join(".local")
                    .join("share")
                    .join("eza-themes")
            })
        })
}

fn get_eza_dir() -> Option<PathBuf> {
    env::var("EZA_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
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
