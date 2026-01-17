mod config;
use config::{Cli, Commands};
mod utils;
use utils::{create_test_dir, merge_yaml_files, vec_list_themes};
mod theme_name;
use theme_name::ThemeName;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result as AnyResult, anyhow};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};

fn main() -> AnyResult<()> {
    let cf = Cli::parse();

    let theme_dir = get_config_dir()?;
    let eza_dir: PathBuf = get_eza_dir()?;
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
    fs::create_dir_all(&theme_dir).context(format!(
        "Failed to create theme directory at {}",
        theme_dir.display()
    ))?;

    // .files are ezt internal files, to distinguish from themes
    let overlay_path = theme_dir.join(".overlay.yml");

    match cf.cmd {
        Commands::List => {
            let v = vec_list_themes(&theme_dir).context("Failed to get themes")?;
            for t in &v {
                print!("{}\n", t.prettify());
            }
        }
        Commands::Switch {
            theme_name,
            interactive,
        } => {
            let theme_name = resolve_theme_name(&theme_dir, interactive, theme_name)
                .context("Failed to select theme name")?;
            let theme_file = ThemeName::from_str(&theme_name)
                .context("Invalid theme name")?
                .to_filename();
            let src = theme_dir.join(&theme_file);
            let dst = eza_dir.join("theme.yml");
            if overlay_path.exists() {
                merge_yaml_files(&src, &overlay_path, &dst)?;
            } else {
                fs::copy(&src, &dst).context(format!(
                    "Failed to copy file {} -> {}",
                    src.display(),
                    dst.display()
                ))?;
            }
        }
        Commands::Add {
            theme_name,
            theme_path,
        } => {
            let dst_theme_file = ThemeName::from_str(&theme_name)
                .context("Invalid theme name")?
                .to_filename();
            let dst = theme_dir.join(&dst_theme_file);
            fs::copy(&theme_path, &dst).context(format!(
                "Failed to copy file {} -> {}",
                theme_path,
                dst.display()
            ))?;
        }
        Commands::Preview {
            theme_name,
            interactive,
        } => {
            let theme_name = resolve_theme_name(&theme_dir, interactive, theme_name)
                .context("Failed to select theme name")?;
            let theme_file = ThemeName::from_str(&theme_name)
                .context("Invalid theme name")?
                .to_filename();
            let src = theme_dir.join(&theme_file);
            if !src.exists() {
                // ![TODO] improve error messages
                println!("{}", theme_file.display());
                return Err(anyhow!("No such file or directory"));
            }

            create_test_dir(&theme_dir).context("Failed to create test dir")?;

            let test_dir = theme_dir.join("test_dir");

            let dst = test_dir.join("theme.yml");
            fs::copy(&src, &dst).context(format!(
                "Failed to copy file {} -> {}",
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

fn get_config_dir() -> AnyResult<PathBuf> {
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
        }).context(concat!(
        "Cannot decide theme directory. (expects as least one of ",
        "EZA_THEME_DIR, XDG_DATA_HOME, HOME to be set)"
        ))

}

fn get_eza_dir() -> AnyResult<PathBuf> {
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
        }).context(concat!(
        "Cannot decide eza directory. (expects as least one of ",
        "EZA_CONFIG_DIR, XDG_CONFIG_HOME, HOME to be set)"
        ))
}

fn resolve_theme_name(
    theme_dir: &Path, 
    is_interactive: bool, maybe_theme_name: Option<String>
) -> AnyResult<String> {
        if is_interactive {
            let themes: Vec<String> = vec_list_themes(theme_dir)
                .context("Failed to get themes")?
                .iter()
                .map(|tn| tn.prettify())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&themes)
                .default(0)
                .interact()?;
            Ok(themes[selection].clone())
        } else {
            // if not interact, theme name is required
            Ok(maybe_theme_name.unwrap())
        }
}
