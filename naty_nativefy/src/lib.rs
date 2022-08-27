use naty_common::{AppSettings, Parser, Platform};
use site_icons::{Icon, IconInfo, IconKind};
use std::path::{Path, PathBuf};

const LINUX: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-linux";
const WIN: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-windows.exe";
const MACOS: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-macos";
const ICON: &[u8] = include_bytes!("../../logos/icon.png");

#[cfg(target_family = "windows")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    let name = format!("{name}.exe");
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

#[cfg(target_family = "unix")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

async fn download_file(
    url: impl AsRef<str>,
    output_dir: impl AsRef<Path>,
    name: &str,
    msg: impl AsRef<str>,
) -> std::io::Result<()> {
    let output_dir = output_dir.as_ref();
    let msg = msg.as_ref();

    let mut downloader = downloader::Downloader::builder()
        .download_folder(output_dir)
        .build()
        .unwrap();
    let download = downloader::Download::new(url.as_ref()).file_name(Path::new(name));
    let output_file = output_dir.join(name);
    if output_file.exists() {
        std::fs::remove_file(output_file)?;
    }

    println!("{msg}");
    let _result =
        tokio::task::spawn_blocking(move || downloader.download(&[download]).unwrap()).await?;
    //println!("{:?}", result);
    Ok(())
}

async fn download_webpage_icon(
    url: impl AsRef<str>,
    output_dir: impl AsRef<Path>,
) -> std::io::Result<()> {
    let output_dir = output_dir.as_ref().to_owned();
    let url = url.as_ref();

    let mut icons = site_icons::Icons::new();
    icons.load_website(url).await.unwrap_or_else(|e| {
        println!("Error");
        match e.downcast::<reqwest_wasm::Error>() {
            Ok(error) => println!("Extracted error: {error}"),
            Err(not) => {
                println!("Not extracted: {}", not);
            }
        }
    });
    let entries = icons.entries().await;
    println!("Available icons: {:?}", entries);
    // Get icon of higher size with: width == height && !Favicon && !SVG
    let mut best_icon: Option<Icon> = None;
    for icon in entries {
        if let Some(sizes) = icon.info.sizes() {
            let size = sizes.first();
            let (width, height) = (size.width, size.height);
            if width == height && icon.kind != IconKind::SiteFavicon && icon.info != IconInfo::SVG {
                best_icon = Some(icon);
                break;
            }
        }
    }

    if let Some(icon) = best_icon {
        let url = icon.url.as_str().to_owned();

        let output_file = output_dir.join("icon.png");
        if output_file.exists() {
            std::fs::remove_file(output_file)?;
        }

        return tokio::task::spawn_blocking(|| {
            println!("Icon Output directory: {}", output_dir.display());
            download_file(url, output_dir, "icon.png", "Downloading icon...")
        })
        .await
        .unwrap()
        .await;
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Website does not have a valid icon",
    ))
}

async fn setup_executable(
    target_url: &str,
    name: Option<&str>,
    output_dir: &Path,
    icon: &[u8],
    naty_executable_url: impl AsRef<str>,
    platform: impl AsRef<str>,
) -> std::io::Result<PathBuf> {
    let naty_exe_url = naty_executable_url.as_ref();
    let mut platform = platform.as_ref();
    if platform.is_empty() {
        platform = std::env::consts::OS;
    }

    let url: url::Url = target_url.try_into().unwrap_or_else(|err| {
        println!("Error parsing the url: {err}");
        std::process::exit(1)
    });
    
    let name = naty_common::get_webpage_name(name, &url);
    let out_dir_name = format!("{}-{platform}", &name);
    let output_dir = output_dir.join(&out_dir_name);
    std::fs::create_dir_all(&output_dir).expect("Could not create directory");

    if platform == std::env::consts::OS {
        copy_executable(&output_dir, &name)?;
    } else {
        let download_url = naty_exe_url.replace("%version%", env!("CARGO_PKG_VERSION"));
        download_file(
            &download_url,
            &output_dir,
            &name,
            format!("Downloading {platform} binary from {download_url}"),
        )
        .await?;
    }
    
    std::fs::write(output_dir.join("icon.png"), icon)?;

    Ok(output_dir)
}

async fn run_async() -> std::io::Result<()> {
    let mut cli: AppSettings = AppSettings::parse();
    
    if cli.platforms.is_empty() {
        cli.platforms.push(std::env::consts::OS.into())
    }
    cli.platforms.dedup();
    
    let icon: std::borrow::Cow<[u8]> = match &cli.icon {
        // Icon is a URL
        Some(url) if download_file(url, &cli.output_dir, "icon.png", "Downloading icon from '{url}'...").await.is_ok() => {
            std::fs::read(cli.output_dir.join("icon.png"))?.into()
        }
        // Icon is a Path
        Some(icon) => {
            std::fs::read(icon).unwrap().into()
        }
        // Icon is extracted from website
        None if download_webpage_icon(&cli.target_url, &cli.output_dir).await.is_ok() => {
            std::fs::read(cli.output_dir.join("icon.png"))?.into()
        }
        // Using fallback icon
        None => {
            println!(
                "Unable to extract an icon from {}, using default one",
                cli.target_url
            );
            ICON.into()
        }
    };
    
    let platforms = cli.platforms.clone();
    for platform in platforms {
        let output_dir = match platform {
            Platform::Linux => {
                cli.command = cli.linux_command.clone();
                setup_executable(&cli.target_url, cli.name.as_deref(), &cli.output_dir, &icon, LINUX,"linux", ).await?
            }
            Platform::Windows => {
                cli.command = cli.windows_command.clone();
                setup_executable(&cli.target_url, cli.name.as_deref(), &cli.output_dir, &icon, WIN, "windows").await?
            }
            Platform::MacOs => {
                cli.command = cli.macos_command.clone();
                setup_executable(&cli.target_url, cli.name.as_deref(), &cli.output_dir, &icon, MACOS, "macos", ).await?
            }
        };

        let settings = toml::to_string_pretty(&cli).unwrap();
        std::fs::write(output_dir.join("naty.toml"), settings).expect("Could not create naty.toml");
        
        println!(
            "Successfully created \"{}\" in {}",
            output_dir.file_name().unwrap().to_string_lossy(),
            output_dir.canonicalize().unwrap().display()
        );
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { run_async().await.unwrap() });
    Ok(())
}
