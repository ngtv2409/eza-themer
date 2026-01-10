use std::{path::Path, fs};
use std::fs::File;
use anyhow::{Result as AnyResult};
/* 
    Create a test directory to preview eza
    Clone of eza-themes/test_dir_
*/
pub fn create_test_dir(base_dir: &Path) -> AnyResult<()> {
    const FILES : [&str; 26] = [
        "Cargo.lock","Cargo.toml","Makefile","README.md","arch.iso","cfg.ini",
        "doc.pdf","file","file.cpp","file.mp4","file.pdf","file.pem","file.png",
        "file.rs","file.tar.gz","file.toml","file.yml","init.sh","justfile",
        "nginx.conf","release.tar.gz","resume.docx","rust.rs","song.flac","song.mp3",
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
