use std::path::PathBuf;
use clap::Parser;

const fn def_false() -> bool {
    false
}
const fn def_true() -> bool {
    true
}
fn def_name() -> String {
    "Application Title".into()
}
const fn def_height() -> u32 {
    800
}
const fn def_width() -> u32 {
    1280
}

#[derive(Parser, serde::Serialize, serde::Deserialize, Debug)]
#[clap(author, version, about)]
pub struct AppSettings {
    /// Url from which the app will be created
    #[clap()]
    pub target_url: String,

    /// Directory where the application will be deployed.
    ///
    /// If not specified, the current directory will be used.
    #[clap(short, long, default_value = ".")]
    #[serde(skip)]
    pub output_dir: PathBuf,

    /// Title of the app
    #[clap(short, long, default_value_t = def_name().into())]
    #[serde(default = "def_name")]
    pub name: String,

    #[clap(short, long, arg_enum)]
    #[serde(skip)]
    pub platforms: Vec<Platform>,

    #[clap(short, long)]
    /// Icon of the app, it must be in a ".png" format
    pub icon: Option<PathBuf>,

    /// Enable always on top window
    #[serde(default = "def_false")]
    #[clap(long)]
    pub always_on_top: bool,

    /// Always start the app in full screen
    #[clap(long)]
    #[serde(default = "def_false")]
    pub full_screen: bool,

    #[clap(long, default_value_t = def_height())]
    #[serde(default = "def_height")]
    pub height: u32,

    #[clap(long, default_value_t = def_width())]
    #[serde(default = "def_width")]
    pub width: u32,

    #[clap(long)]
    #[serde(default = "def_false")]
    pub hide_window_frame: bool,

    #[clap(long)]
    #[serde(default = "def_false")]
    pub show_menu_bar: bool,

    #[clap(long, default_value_t = u32::MAX)]
    #[serde(default = "u32::max_value")]
    pub max_width: u32,
    #[clap(long, default_value_t = u32::MAX)]
    #[serde(default = "u32::max_value")]
    pub max_height: u32,

    #[clap(long, default_value_t = u32::MIN)]
    #[serde(default = "u32::min_value")]
    pub min_width: u32,
    #[clap(long, default_value_t = u32::MIN)]
    #[serde(default = "u32::min_value")]
    pub min_height: u32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, clap::ArgEnum)]
pub enum Platform {
    Linux,
    Windows,
    MacOs,
}

impl From<&str> for Platform {
    fn from(p: &str) -> Self {
        match p {
            "linux" => Platform::Linux,
            "windows" => Platform::Windows,
            _ => Platform::MacOs
        }
    }
}