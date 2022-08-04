use crate::application::{def_height, def_width};
use clap::{clap_derive::ArgEnum, Parser};
use std::path::{Path, PathBuf};

pub const fn def_name() -> &'static str { "Application Title" }

#[derive(Parser, Debug, serde::Serialize)]
#[clap(author, version, about)]
struct Cli {
    /// Url from which the app will be created
    #[clap()]
    target_url: String,

    /// Directory where the application will be deployed.
    ///
    /// If not specified, the current directory will be used.
    #[serde(skip)]
    #[clap(short, long, default_value = ".")]
    output_dir: PathBuf,

    /// Title of the app
    #[clap(short, long, default_value_t = def_name().into())]
    name: String,

    #[clap(short, long, arg_enum)]
    platforms: Vec<Platform>,

    #[clap(short, long)]
    /// Icon of the app, it must be in a ".png" format
    icon: Option<PathBuf>,

    /// Enable always on top window
    #[clap(long)]
    always_on_top: bool,
    
    /// Always start the app in full screen
    #[clap(long)]
    full_screen: bool,

    #[clap(long, default_value_t = def_height())]
    height: u32,
    
    #[clap(long, default_value_t = def_width())]
    width: u32,
    
    #[clap(long)]
    hide_window_frame: bool,
    
    #[clap(long)]
    show_menu_bar: bool,
    
    #[clap(long, default_value_t = u32::MAX)]
    max_width: u32,
    #[clap(long, default_value_t = u32::MAX)]
    max_height: u32,
    
    #[clap(long, default_value_t = u32::MIN)]
    min_width: u32,
    #[clap(long, default_value_t = u32::MIN)]
    min_height: u32,

}

#[derive(Clone, Debug, PartialEq, serde::Serialize, ArgEnum)]
enum Platform {
    Linux,
    Windows,
    MacOs,
}

const LINUX: &str = "https://github.com/LyonSyonII/naty/releases/download/v0.1.4/naty-linux";
const WIN: &str = "https://github.com/LyonSyonII/naty/releases/download/v0.1.4/naty-windows";
const MACOS: &str = "https://github.com/LyonSyonII/naty/releases/download/v0.1.4/naty-macos";

#[cfg(target_family = "windows")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    let name = format!("{name}.exe");
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

#[cfg(target_family = "unix")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

fn download_executable(url: &str, output_dir: impl AsRef<Path>, name: impl AsRef<str>, msg: impl AsRef<str>) -> std::io::Result<()> {
    let output_dir = output_dir.as_ref();
    let name = name.as_ref();
    let msg = msg.as_ref();

    let mut downloader = downloader::Downloader::builder().download_folder(output_dir).build().unwrap();
    let download = downloader::Download::new(url).file_name(Path::new(name));
    
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

pub fn run() -> std::io::Result<()> {
    let cli: Cli = Cli::parse();
    
    let settings = toml::to_string_pretty(&cli).unwrap();
    let output_dir = cli.output_dir.join(&cli.name);
    std::fs::create_dir_all(&output_dir).expect("Could not create directory");
    std::fs::write(output_dir.join("naty.toml"), settings).expect("Could not create naty.toml");
    if let Some(icon) = cli.icon {
        std::fs::copy(&icon, output_dir.join(icon.file_name().unwrap())).expect("Could not copy icon");
    }
    
    if cli.platforms.is_empty() {
        copy_executable(&output_dir, &cli.name)?;
    } else {
        let mut platforms = cli.platforms;
        platforms.dedup();
        for platform in platforms {
            match platform {
                Platform::Linux => download_executable(LINUX, &output_dir, &cli.name, "Downloading Linux binary...")?,
                Platform::Windows => download_executable(WIN, &output_dir, format!("{}.exe", cli.name), "Downloading Windows binary...")?,
                Platform::MacOs => download_executable(MACOS, &output_dir, &cli.name, "Downloading MacOS binary...")?,
            }
        }
    }
    
    Ok(())
    
    
}
