use std::path::Path;
use clap::Parser;

use crate::structs::{AppSettings, Platform};

const LINUX: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-linux";
const WIN: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-windows.exe";
const MACOS: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-macos";

#[cfg(target_family = "windows")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    let name = format!("{name}.exe");
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

#[cfg(target_family = "unix")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

fn download_executable(
    url: impl AsRef<str>,
    output_dir: impl AsRef<Path>,
    name: impl AsRef<str>,
    msg: impl AsRef<str>,
) -> std::io::Result<()> {
    let url = url.as_ref();
    let output_dir = output_dir.as_ref();
    let name = name.as_ref();
    let msg = msg.as_ref();

    let mut downloader = downloader::Downloader::builder()
        .download_folder(output_dir)
        .build()
        .unwrap();
    let download = downloader::Download::new(&url.replace("%version%", env!("CARGO_PKG_VERSION")))
        .file_name(Path::new(name));

    let output_file = output_dir.join(name);
    if output_file.exists() {
        std::fs::remove_file(output_file)?;
    }

    println!("{msg}");
    let result = downloader.download(&[download]).unwrap();

    println!("{:?}", result);

    Ok(())
}

// pub fn download_linux_executable(output_dir: &Path, name: &str) -> std::io::Result<()> {
//     download_executable(output_dir, name, "Downloading Linux binary...")
// }
//
// pub fn download_windows_executable(output_dir: &Path, name: &str) -> std::io::Result<()> {
//     download_executable(output_dir, format!("{name}.exe"), "Downloading Windows binary...")
// }
//
// pub fn download_macos_executable(output_dir: &Path, name: &str) -> std::io::Result<()> {
//     download_executable(output_dir, name, "Downloading MacOS binary...")
// }

fn setup_executable(
    cli: &AppSettings,
    settings: impl AsRef<str>,
    url: impl AsRef<str>,
    platform: impl AsRef<str>,
) -> std::io::Result<()> {
    let url = url.as_ref();
    let mut platform = platform.as_ref();
    if platform.is_empty() {
        platform = std::env::consts::OS;
    }

    let output_dir = cli.output_dir.join(format!("{}-{platform}", cli.name));
    std::fs::create_dir_all(&output_dir).expect("Could not create directory");
    std::fs::write(output_dir.join("naty.toml"), settings.as_ref())
        .expect("Could not create naty.toml");
    if let Some(icon) = &cli.icon {
        std::fs::copy(&icon, output_dir.join(icon.file_name().unwrap()))
            .expect("Could not copy icon");
    }

    if platform == std::env::consts::OS {
        copy_executable(&output_dir, &cli.name)?;
    } else {
        download_executable(
            url,
            output_dir,
            &cli.name,
            format!("Downloading {platform} binary..."),
        )?;
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let mut cli: AppSettings = AppSettings::parse();
    let settings = toml::to_string_pretty(&cli).unwrap();

    if cli.platforms.is_empty() {
        cli.platforms.push(std::env::consts::OS.into())
    }
    cli.platforms.dedup();

    for platform in &cli.platforms {
        match platform {
            Platform::Linux => {
                setup_executable(&cli, &settings, LINUX, "linux")?;
            }
            Platform::Windows => {
                setup_executable(&cli, &settings, WIN, "windows")?;
            }
            Platform::MacOs => {
                setup_executable(&cli, &settings, MACOS, "macos")?;
            }
        }
    }

    Ok(())
}
