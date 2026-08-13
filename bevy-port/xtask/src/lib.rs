use std::{
    fs::{self, File},
    io::{Seek, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

pub const WINDOWS_PACKAGE_NAME: &str = "stream-town-windows-x86_64.zip";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageReport {
    pub archive: PathBuf,
    pub files: usize,
    pub bytes: u64,
}

pub fn package_windows(workspace: &Path, output: &Path, skip_build: bool) -> Result<PackageReport> {
    if !cfg!(target_os = "windows") {
        bail!("Windows packaging must run on Windows");
    }
    let workspace = workspace
        .canonicalize()
        .with_context(|| format!("failed to resolve workspace {}", workspace.display()))?;
    if !skip_build {
        let status = std::process::Command::new("cargo")
            .current_dir(&workspace)
            .args([
                "build",
                "--release",
                "-p",
                "stream_town_game",
                "-p",
                "stream_town_tools",
            ])
            .status()
            .context("failed to start Cargo release build")?;
        if !status.success() {
            bail!("Cargo release build failed with {status}");
        }
    }
    let game = workspace.join("target/release/stream_town_game.exe");
    let tools = workspace.join("target/release/stream_town_tools.exe");
    for executable in [&game, &tools] {
        if !executable.is_file() {
            bail!(
                "release executable is missing: {}; build without --skip-build",
                executable.display()
            );
        }
    }
    let assets = workspace.join("assets");
    if !assets.is_dir() {
        bail!("packaged asset root is missing: {}", assets.display());
    }
    let repository = workspace
        .parent()
        .context("Bevy workspace has no repository parent")?;
    let license = repository.join("LICENSE");
    if !license.is_file() {
        bail!("GPL license is missing: {}", license.display());
    }
    let readme = workspace.join("README.md");
    if !readme.is_file() {
        bail!("Bevy README is missing: {}", readme.display());
    }

    fs::create_dir_all(output)
        .with_context(|| format!("failed to create package directory {}", output.display()))?;
    let archive = output.join(WINDOWS_PACKAGE_NAME);
    let temporary = archive.with_extension("zip.tmp");
    if temporary.is_file() {
        fs::remove_file(&temporary)?;
    }
    let file = File::create(&temporary)
        .with_context(|| format!("failed to create package {}", temporary.display()))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let mut files = 0_usize;
    let mut bytes = 0_u64;
    add_file(
        &mut zip,
        &game,
        "StreamTown/stream_town_game.exe",
        options,
        &mut files,
        &mut bytes,
    )?;
    add_file(
        &mut zip,
        &tools,
        "StreamTown/stream_town_tools.exe",
        options,
        &mut files,
        &mut bytes,
    )?;
    add_file(
        &mut zip,
        &license,
        "StreamTown/LICENSE",
        options,
        &mut files,
        &mut bytes,
    )?;
    add_file(
        &mut zip,
        &readme,
        "StreamTown/README.md",
        options,
        &mut files,
        &mut bytes,
    )?;
    for entry in WalkDir::new(&assets).sort_by_file_name() {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry.path().strip_prefix(&workspace)?;
        let archive_path = format!("StreamTown/{}", portable_path(relative));
        add_file(
            &mut zip,
            entry.path(),
            &archive_path,
            options,
            &mut files,
            &mut bytes,
        )?;
    }
    zip.finish()?.sync_all()?;
    if archive.is_file() {
        fs::remove_file(&archive)?;
    }
    fs::rename(&temporary, &archive).with_context(|| {
        format!(
            "failed to atomically publish {} as {}",
            temporary.display(),
            archive.display()
        )
    })?;
    validate_windows_package(&archive)?;
    Ok(PackageReport {
        archive,
        files,
        bytes,
    })
}

pub fn validate_windows_package(archive: &Path) -> Result<()> {
    let mut zip = zip::ZipArchive::new(
        File::open(archive)
            .with_context(|| format!("failed to open package {}", archive.display()))?,
    )?;
    for required in [
        "StreamTown/stream_town_game.exe",
        "StreamTown/stream_town_tools.exe",
        "StreamTown/LICENSE",
        "StreamTown/README.md",
        "StreamTown/assets/config/game.ron",
        "StreamTown/assets/config/player-settings.ron",
        "StreamTown/assets/content/catalog.ron",
        "StreamTown/assets/content/presentation.ron",
    ] {
        zip.by_name(required)
            .with_context(|| format!("package is missing {required}"))?;
    }
    for index in 0..zip.len() {
        let file = zip.by_index(index)?;
        if file.is_dir() {
            continue;
        }
        let path = Path::new(file.name());
        if path.is_absolute()
            || path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            bail!("package contains unsafe path {}", file.name());
        }
    }
    Ok(())
}

fn add_file<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    source: &Path,
    archive_path: &str,
    options: SimpleFileOptions,
    files: &mut usize,
    bytes: &mut u64,
) -> Result<()> {
    zip.start_file(archive_path, options)?;
    let mut source_file = File::open(source)
        .with_context(|| format!("failed to open package input {}", source.display()))?;
    let copied = std::io::copy(&mut source_file, zip)?;
    *files = files.saturating_add(1);
    *bytes = bytes.saturating_add(copied);
    Ok(())
}

fn portable_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            std::path::Component::Normal(part) => Some(part.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_name_and_paths_are_portable() {
        assert_eq!(WINDOWS_PACKAGE_NAME, "stream-town-windows-x86_64.zip");
        assert_eq!(
            portable_path(Path::new("assets/config/game.ron")),
            "assets/config/game.ron"
        );
        assert_eq!(
            portable_path(Path::new("assets/config/player-settings.ron")),
            "assets/config/player-settings.ron"
        );
    }
}
