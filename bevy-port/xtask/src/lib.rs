use std::{
    fs::{self, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha512};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

pub const WINDOWS_PACKAGE_NAME: &str = "stream-town-windows-x86_64.zip";
const FFMPEG_SOURCE_SHA512: &str = "e858e92e5eb08d562302cde371af55917df6e1fe53994e18462a3c929a40ede1828c2bd53c2a7d65a2cfd791782ead3cd94efb2def904f49cb5dd8ab5cd4256f";
const OPENH264_SOURCE_SHA512: &str = "26a03acde7153a6b40b99f00641772433a244c72a3cc4bca6d903cf3b770174d028369a2fb73b2f0774e1124db0e269758eed6d88975347a815e0366c820d247";
const AMD_AMF_SOURCE_SHA512: &str = "b992d4a1f59f7b1c789d03e7bd9876417a569fb239bfe2e2178f2434ae18653bbacc912de2b8a5f8ff0a85fad28b0c1091c2a8d3417407a37c22c1e907e4c159";

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
    let workspace = dunce::canonicalize(workspace)
        .with_context(|| format!("failed to resolve workspace {}", workspace.display()))?;
    let vcpkg_root = std::env::var_os("VCPKG_ROOT")
        .map(PathBuf::from)
        .context("VCPKG_ROOT is required to package the shared FFmpeg runtime")?;
    let installed_root = std::env::var_os("VCPKG_INSTALLED_ROOT").map_or_else(
        || {
            let manifest = workspace.join("vcpkg_installed");
            if manifest.join("x64-windows").is_dir() {
                manifest
            } else {
                vcpkg_root.join("installed")
            }
        },
        PathBuf::from,
    );
    let native_root = installed_root.join("x64-windows");
    if !skip_build {
        if ffmpeg_link_metadata_is_stale(&workspace, &native_root)? {
            let status = std::process::Command::new("cargo")
                .current_dir(&workspace)
                .args(["clean", "--release", "-p", "ffmpeg-sys-next"])
                .status()
                .context("failed to clear stale FFmpeg release metadata")?;
            if !status.success() {
                bail!("clearing stale FFmpeg release metadata failed with {status}");
            }
        }
        let status = std::process::Command::new("cargo")
            .current_dir(&workspace)
            // ffmpeg-sys' generic vcpkg probe inherits transitive pkg-config
            // libraries, including optional modules that this manifest does not
            // install. Point it at the exact dynamic-library prefix so its own
            // feature-aware linker list remains authoritative.
            .env("FFMPEG_DIR", &native_root)
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
    let ffmpeg_notice = workspace.join("third_party/ffmpeg/README.md");
    if !ffmpeg_notice.is_file() {
        bail!(
            "FFmpeg relinking notice is missing: {}",
            ffmpeg_notice.display()
        );
    }
    let native_bin = native_root.join("bin");
    let ffmpeg_dlls = [
        "avcodec-62.dll",
        "avformat-62.dll",
        "avutil-60.dll",
        "swresample-6.dll",
        "swscale-9.dll",
        "openh264-7.dll",
    ]
    .map(|name| native_bin.join(name));
    for dll in &ffmpeg_dlls {
        if !dll.is_file() {
            bail!(
                "shared direct-broadcast dependency is missing: {}; run the pinned vcpkg install",
                dll.display()
            );
        }
    }
    let downloads = std::env::var_os("VCPKG_DOWNLOADS")
        .map_or_else(|| vcpkg_root.join("downloads"), PathBuf::from);
    let ffmpeg_source = downloads.join("ffmpeg-ffmpeg-n8.1.1.tar.gz");
    let openh264_source = downloads.join("cisco-openh264-v2.6.0.tar.gz");
    let amd_amf_source = downloads.join("AMF-headers-v1.5.2.tar.gz");
    for source in [&ffmpeg_source, &openh264_source, &amd_amf_source] {
        if !source.is_file() {
            bail!(
                "corresponding native-library source archive is missing: {}",
                source.display()
            );
        }
    }
    verify_sha512(&ffmpeg_source, FFMPEG_SOURCE_SHA512)?;
    verify_sha512(&openh264_source, OPENH264_SOURCE_SHA512)?;
    verify_sha512(&amd_amf_source, AMD_AMF_SOURCE_SHA512)?;

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
        &ffmpeg_notice,
        "StreamTown/third_party/FFMPEG_RELINKING.md",
        options,
        &mut files,
        &mut bytes,
    )?;
    for dll in &ffmpeg_dlls {
        let name = dll
            .file_name()
            .context("native dependency has no filename")?
            .to_string_lossy();
        add_file(
            &mut zip,
            dll,
            &format!("StreamTown/{name}"),
            options,
            &mut files,
            &mut bytes,
        )?;
    }
    add_file(
        &mut zip,
        &ffmpeg_source,
        "StreamTown/third_party/source/ffmpeg-ffmpeg-n8.1.1.tar.gz",
        options,
        &mut files,
        &mut bytes,
    )?;
    add_file(
        &mut zip,
        &openh264_source,
        "StreamTown/third_party/source/cisco-openh264-v2.6.0.tar.gz",
        options,
        &mut files,
        &mut bytes,
    )?;
    add_file(
        &mut zip,
        &amd_amf_source,
        "StreamTown/third_party/source/AMF-headers-v1.5.2.tar.gz",
        options,
        &mut files,
        &mut bytes,
    )?;
    for package in ["ffmpeg", "openh264", "amd-amf"] {
        let share = native_root.join("share").join(package);
        if !share.is_dir() {
            bail!("vcpkg package metadata is missing: {}", share.display());
        }
        add_tree(
            &mut zip,
            &share,
            &format!("StreamTown/third_party/vcpkg-installed/{package}"),
            options,
            &mut files,
            &mut bytes,
        )?;
        let port = vcpkg_root.join("ports").join(package);
        if !port.is_dir() {
            bail!("vcpkg port recipe is missing: {}", port.display());
        }
        add_tree(
            &mut zip,
            &port,
            &format!("StreamTown/third_party/vcpkg-ports/{package}"),
            options,
            &mut files,
            &mut bytes,
        )?;
    }
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

fn ffmpeg_link_metadata_is_stale(workspace: &Path, native_root: &Path) -> Result<bool> {
    let build_root = workspace.join("target/release/build");
    if !build_root.is_dir() {
        return Ok(false);
    }
    let expected_search = format!(
        "cargo:rustc-link-search=native={}",
        native_root.join("lib").display()
    );
    for entry in fs::read_dir(&build_root)
        .with_context(|| format!("failed to inspect {}", build_root.display()))?
    {
        let entry = entry?;
        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("ffmpeg-sys-next-")
        {
            continue;
        }
        let output = entry.path().join("output");
        if !output.is_file() {
            continue;
        }
        let metadata = fs::read_to_string(&output)
            .with_context(|| format!("failed to read {}", output.display()))?;
        if !metadata.lines().any(|line| line == expected_search)
            || metadata.lines().any(|line| {
                matches!(
                    line,
                    "cargo:rustc-link-lib=avdevice" | "cargo:rustc-link-lib=avfilter"
                )
            })
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn add_tree<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    source_root: &Path,
    archive_root: &str,
    options: SimpleFileOptions,
    files: &mut usize,
    bytes: &mut u64,
) -> Result<()> {
    for entry in WalkDir::new(source_root).sort_by_file_name() {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry.path().strip_prefix(source_root)?;
        let archive_path = format!("{archive_root}/{}", portable_path(relative));
        add_file(zip, entry.path(), &archive_path, options, files, bytes)?;
    }
    Ok(())
}

fn verify_sha512(path: &Path, expected: &str) -> Result<()> {
    let mut file = File::open(path)
        .with_context(|| format!("failed to open source archive {}", path.display()))?;
    let mut digest = Sha512::new();
    let mut buffer = vec![0_u8; 64 * 1_024];
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("failed to hash source archive {}", path.display()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    let actual = format!("{:x}", digest.finalize());
    if actual != expected {
        bail!(
            "source archive {} has SHA-512 {actual}, expected {expected}",
            path.display()
        );
    }
    Ok(())
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
        "StreamTown/avcodec-62.dll",
        "StreamTown/avformat-62.dll",
        "StreamTown/avutil-60.dll",
        "StreamTown/swresample-6.dll",
        "StreamTown/swscale-9.dll",
        "StreamTown/openh264-7.dll",
        "StreamTown/third_party/FFMPEG_RELINKING.md",
        "StreamTown/third_party/source/ffmpeg-ffmpeg-n8.1.1.tar.gz",
        "StreamTown/third_party/source/cisco-openh264-v2.6.0.tar.gz",
        "StreamTown/third_party/source/AMF-headers-v1.5.2.tar.gz",
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
