pub use clap::Parser;
use std::path::{PathBuf};
use url::Url;

const fn def_false() -> bool {
    false
}
#[allow(dead_code)]
const fn def_true() -> bool {
    true
}
const fn def_height() -> u32 {
    800
}
const fn def_width() -> u32 {
    1280
}

#[derive(serde::Deserialize, serde::Serialize, Parser, Debug, Clone)]
#[clap(name = "naty", author, version, about)]
pub struct AppSettings {
    /// The URL that you wish to to turn into a native app.
    #[clap()]
    #[serde()]
    pub target_url: String,
    
    /// The directory to generate the app in.
    ///
    /// If not specified, the current directory will be used.
    #[clap(short, long, default_value = ".")]
    #[serde(skip)]
    pub output_dir: PathBuf,
    
    /// Title of the app.
    ///
    /// If not specified, Naty will try to extract it from the TARGET_URL.
    #[clap(short, long)]
    #[serde()]
    pub name: Option<String>,
    
    /// The operating systems to build for.
    #[clap(short, long, value_enum)]
    #[serde(skip)]
    pub platforms: Vec<Platform>,
    
    /// Icon of the app, must be a ".png" file.
    ///
    /// Can be either a path to a local file or a URL.
    /// 
    /// If not provided, Naty will try to extract it from the TARGET_URL.
    #[clap(short, long)]
    #[serde(skip)]
    pub icon: Option<String>,
    
    /// Enable always on top window.
    #[clap(long)]
    #[serde(default = "def_false")]
    pub always_on_top: bool,
    
    /// Always start the app in full screen.
    #[clap(long)]
    #[serde(default = "def_false")]
    pub full_screen: bool,

    /// App window default height in pixels.
    #[clap(long, default_value_t = def_height())]
    #[serde(default = "def_height")]
    pub height: u32,

    /// App window default width in pixels.
    #[clap(long, default_value_t = def_width())]
    #[serde(default = "def_width")]
    pub width: u32,

    /// Disable window frame and controls.
    #[clap(long)]
    #[serde(default = "def_false")]
    pub hide_window_frame: bool,
    
    /// WIP: At the moment it does nothing.
    #[clap(long)]
    #[serde(default = "def_false")]
    pub show_menu_bar: bool,

    /// Set window maximum width in pixels.
    #[clap(long, default_value_t = u32::MAX)]
    #[serde(default = "u32::max_value")]
    pub max_width: u32,
    
    /// Set window maximum height in pixels.
    #[clap(long, default_value_t = u32::MAX)]
    #[serde(default = "u32::max_value")]
    pub max_height: u32,
    
    /// Set window minimum width in pixels.
    #[clap(long, default_value_t = u32::MIN)]
    #[serde(default = "u32::min_value")]
    pub min_width: u32,

    /// Set window minimum height in pixels.
    #[clap(long, default_value_t = u32::MIN)]
    #[serde(default = "u32::min_value")]
    pub min_height: u32,
    
    /// Hides taskbar icon.
    ///
    /// The window can't be minimized.
    #[clap(long)]
    pub hide_taskbar_icon: bool,
    
    /// LINUX: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[clap(long = "command_linux")]
    #[serde(skip)]
    pub linux_command: Option<String>,
    
    /// WINDOWS: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[clap(long = "command_windows")]
    #[serde(skip)]
    pub windows_command: Option<String>,
    
    /// MACOS: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[clap(long = "command_macos")]
    #[serde(skip)]
    pub macos_command: Option<String>,
    
    /// Command to run before spawning a window, useful for starting web servers for WebApps.
    pub command: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
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
            _ => Platform::MacOs,
        }
    }
}

pub fn maybe_remove<'i>(
    original: impl Into<std::borrow::Cow<'i, str>>,
    needles: impl AsRef<[&'i str]>,
) -> std::borrow::Cow<'i, str> {
    let mut result: std::borrow::Cow<'i, str> = original.into();
    for needle in needles.as_ref() {
        let find = result.find(needle);
        if let Some(start) = find {
            let mut tmp = std::borrow::Cow::into_owned(result);
            tmp.replace_range(start..needle.len(), "");
            result = tmp.into();
        }
    }
    result
}

pub fn get_webpage_name<'i>(name: Option<&'i str>, url: &'i Url) -> std::borrow::Cow<'i, str> {
    if let Some(name) = name {
        return name.into();
    }

    url.domain()
        .and_then(|domain| domain.rfind('.').and_then(|idx| domain[..idx].into()))
        .unwrap_or("App Name")
        .into()
}

pub fn get_exe_path() -> PathBuf {
    std::env::current_exe().unwrap().parent().unwrap().into()
}
