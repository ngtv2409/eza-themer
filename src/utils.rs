use crate::theme_name::ThemeName;
use anyhow::Result as AnyResult;
use serde_yaml::{Mapping, Value};
use std::fs::File;
use std::{fs, path::Path};

/*
    Create a test directory to preview eza
    Clone of eza-themes/test_dir_
*/
pub fn create_test_dir(base_dir: &Path) -> AnyResult<()> {
    const FILES: [&str; 26] = [
        "Cargo.lock",
        "Cargo.toml",
        "Makefile",
        "README.md",
        "arch.iso",
        "cfg.ini",
        "doc.pdf",
        "file",
        "file.cpp",
        "file.mp4",
        "file.pdf",
        "file.pem",
        "file.png",
        "file.rs",
        "file.tar.gz",
        "file.toml",
        "file.yml",
        "init.sh",
        "justfile",
        "nginx.conf",
        "release.tar.gz",
        "resume.docx",
        "rust.rs",
        "song.flac",
        "song.mp3",
        "source.cpp",
    ];

    let test_dir = base_dir.join("test_dir");
    fs::create_dir_all(&test_dir)?;
    fs::create_dir_all(&test_dir.join("src"))?;

    for file in &FILES {
        let path = test_dir.join(file);

        File::create(path)?;
    }

    Ok(())
}

/// Merge `overlay` into `base`  
/// - Scalars: overwrite  
/// - Lists: append  
/// - Maps: recursive merge  
pub fn merge_yaml(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Mapping(base_map), Value::Mapping(overlay_map)) => {
            for (k, v) in overlay_map {
                if let Some(base_val) = base_map.get_mut(k) {
                    merge_yaml(base_val, v);
                } else {
                    base_map.insert(k.clone(), v.clone());
                }
            }
        }
        (Value::Sequence(base_seq), Value::Sequence(overlay_seq)) => {
            base_seq.extend(overlay_seq.clone());
        }
        (base_val, overlay_val) => {
            *base_val = overlay_val.clone(); // overwrite scalars  
        }
    }
}

pub fn merge_yaml_files(base_path: &Path, overlay_path: &Path, dst: &Path) -> AnyResult<()> {
    let base_yaml: Value = if base_path.exists() {
        let s = fs::read_to_string(base_path)?;
        if s.trim().is_empty() {
            Value::Mapping(Mapping::new())
        } else {
            serde_yaml::from_str(&s)?
        }
    } else {
        Value::Mapping(Mapping::new())
    };

    let overlay_yaml: Value = if overlay_path.exists() {
        let s = fs::read_to_string(overlay_path)?;
        if s.trim().is_empty() {
            Value::Mapping(Mapping::new())
        } else {
            serde_yaml::from_str(&s)?
        }
    } else {
        Value::Mapping(Mapping::new())
    };

    let mut merged = base_yaml;
    merge_yaml(&mut merged, &overlay_yaml);

    fs::write(dst, serde_yaml::to_string(&merged)?)?;

    Ok(())
}

pub fn vec_list_themes(dir: &Path) -> AnyResult<Vec<ThemeName>> {
    let mut v = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        // Skip dotfiles
        if filename_str.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yml" {
                    if let Some(tn) =
                        ThemeName::from_str(&filename_str.strip_suffix(".yml").unwrap())
                    {
                        v.push(tn);
                    }
                }
            }
        }
    }
    Ok(v)
}
