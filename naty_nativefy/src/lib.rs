use naty_common::{Parser, AppSettings, Platform};
use std::path::Path;

const LINUX: &str = "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-linux";
const WIN: &str =
    "https://github.com/LyonSyonII/naty/releases/download/v%version%/naty-windows.exe";
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
    let result = tokio::task::spawn_blocking(move || { downloader.download(&[download]).unwrap() }).await;
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

async fn download_webpage_icon(url: impl AsRef<str>, output_dir: impl AsRef<Path>) -> std::io::Result<()> {
    let output_dir = output_dir.as_ref().to_owned();
    let url = url.as_ref();

    let mut icons = site_icons::Icons::new();
    icons.load_website(url).await.unwrap_or_else(|e| {
        println!("Error");
        // fn type_of<T>(_: &T) -> &'static str {
        //     std::any::type_name::<T>()
        // }
        // println!("{}", type_of(&e));
        // Does not work
        match e.downcast::<reqwest_wasm::Error>() {
            Ok(error) => println!("Extracted error: {error}"),
            Err(not) => { println!("Not extracted: {}", not); },
        }
    });
    let entries = icons.entries().await;
    
    if let Some(icon) = entries.first() {
        let url = icon.url.as_str().to_owned();
        
        let output_file = output_dir.join("icon");
        if output_file.exists() {
            std::fs::remove_file(output_file)?;
        }

        return tokio::task::spawn_blocking(|| {
            println!("Icon Output directory: {}", output_dir.display());
            download_file(url, output_dir, "icon", "Downloading icon...")
        }).await.unwrap().await;
    }
    
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Website does not have an icon"))
}

async fn setup_executable(
    cli: &AppSettings,
    download_url: impl AsRef<str>,
    platform: impl AsRef<str>,
) -> std::io::Result<()> {
    let mut cli = cli.clone();
    let download_url = download_url.as_ref();
    let mut platform = platform.as_ref();
    if platform.is_empty() {
        platform = std::env::consts::OS;
    }
    
    let name = naty_common::get_webpage_name(cli.name.as_ref(), &cli.target_url);
    let out_dir_name = format!("{}-{platform}", &name);
    let output_dir = cli.output_dir.join(&out_dir_name);
    std::fs::create_dir_all(&output_dir).expect("Could not create directory");

    if let Some(icon) = &cli.icon {
        std::fs::copy(&icon, output_dir.join(icon.file_name().unwrap()))
            .expect("Could not copy icon");
    } else if download_webpage_icon(&cli.target_url, &output_dir).await.is_ok() {
        cli.icon = Some("icon".into())
    } else {
        println!("Unable to extract an icon from {}, using default one", cli.target_url)
    }

    if platform == std::env::consts::OS {
        copy_executable(&output_dir, &name)?;
    } else {
        let download_url = download_url.replace("%version%", env!("CARGO_PKG_VERSION"));
        download_file(
            &download_url,
            &output_dir,
            &name,
            format!("Downloading {platform} binary from {download_url}"),
        ).await?;
    }
    
    let settings = toml::to_string_pretty(&cli).unwrap();
    std::fs::write(output_dir.join("naty.toml"), settings).expect("Could not create naty.toml");

    println!(
        "Successfully created \"{out_dir_name}\" in {}",
        output_dir.canonicalize().unwrap().display()
    );

    Ok(())
}

async fn run_async() -> std::io::Result<()> {
    let mut cli: AppSettings = AppSettings::parse();

    if cli.platforms.is_empty() {
        cli.platforms.push(std::env::consts::OS.into())
    }
    cli.platforms.dedup();

    for platform in &cli.platforms {
        match platform {
            Platform::Linux => {
                setup_executable(&cli, LINUX, "linux").await?;
            }
            Platform::Windows => {
                setup_executable(&cli, WIN, "windows").await?;
            }
            Platform::MacOs => {
                setup_executable(&cli, MACOS, "macos").await?;
            }
        }
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { run_async().await.unwrap() });
    Ok(())
}