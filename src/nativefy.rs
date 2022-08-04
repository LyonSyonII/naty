use crate::application::{def_height, def_width};
use clap::{clap_derive::ArgEnum, Parser};
use std::path::{Path, PathBuf};

pub fn def_out_dir() -> PathBuf { std::env::current_dir().unwrap() }
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

#[cfg(target_family = "windows")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    let name = format!("{name}.exe");
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

#[cfg(target_family = "unix")]
pub fn copy_executable(output_dir: &Path, name: &str) -> std::io::Result<u64> {
    std::fs::copy(std::env::current_exe()?, output_dir.join(name))
}

pub fn run() -> std::io::Result<()> {
    let cli: Cli = Cli::parse();
    println!("{cli:?}");
    
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
        // for platform in platforms {
        //     match platform {
        //         Platform::Linux => copy_linux_executable(),
        //         Platform::Windows => copy_windows_executable(),
        //         Platform::MacOs => copy_macos_executable(),
        //     }
        // }
    }
    
    Ok(())
    
    
}
