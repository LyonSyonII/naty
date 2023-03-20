use std::path::PathBuf;
use url::Url;

const fn def_height() -> u32 {
    800
}
const fn def_width() -> u32 {
    1280
}

#[cfg(feature = "clap")]
pub use clap::Parser;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", clap(name = "naty", author, version, about))]
pub struct AppSettings {
    /// The URL that you wish to to turn into a native app.
    #[cfg_attr(feature = "clap", clap())]
    pub target_url: String,

    /// The directory to generate the app in.
    ///
    /// If not specified, the current directory will be used.
    #[cfg_attr(feature = "clap", clap(short, long, default_value = "."))]
    #[serde(skip)]
    pub output_dir: PathBuf,

    /// Title of the app.
    ///
    /// If not specified, Naty will try to extract it from the TARGET_URL.
    #[cfg_attr(feature = "clap", clap(short, long))]
    pub name: Option<String>,

    /// The operating systems to build for.
    #[cfg_attr(feature = "clap", clap(short, long, value_enum))]
    #[serde(skip)]
    pub platforms: Vec<Platform>,

    /// Icon of the app, must be a ".png" file.
    ///
    /// Can be either a path to a local file or a URL.
    ///
    /// If not provided, Naty will try to extract it from the TARGET_URL.
    #[cfg_attr(feature = "clap", clap(short, long))]
    #[serde(skip)]
    pub icon: Option<String>,

    /// Enable always on top window.
    #[cfg_attr(feature = "clap", clap(long))]
    #[serde(default)]
    pub always_on_top: bool,

    /// Always start the app in full screen.
    #[cfg_attr(feature = "clap", clap(long))]
    #[serde(default)]
    pub full_screen: bool,

    /// App window default height in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = def_height()))]
    #[serde(default = "def_height")]
    pub height: u32,

    /// App window default width in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = def_width()))]
    #[serde(default = "def_width")]
    pub width: u32,

    /// Disable window frame and controls.
    #[cfg_attr(feature = "clap", clap(long))]
    #[serde(default)]
    pub hide_window_frame: bool,

    /// WIP: At the moment it does nothing.
    #[cfg_attr(feature = "clap", clap(long))]
    #[serde(default)]
    pub show_menu_bar: bool,

    /// Set window maximum width in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = u32::MAX))]
    #[serde(default = "u32::max_value")]
    pub max_width: u32,

    /// Set window maximum height in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = u32::MAX))]
    #[serde(default = "u32::max_value")]
    pub max_height: u32,

    /// Set window minimum width in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = u32::MIN))]
    #[serde(default = "u32::min_value")]
    pub min_width: u32,

    /// Set window minimum height in pixels.
    #[cfg_attr(feature = "clap", clap(long, default_value_t = u32::MIN))]
    #[serde(default = "u32::min_value")]
    ///
    pub min_height: u32,

    /// LINUX: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[cfg_attr(feature = "clap", clap(long = "command-linux"))]
    #[serde(skip)]
    pub linux_command: Option<String>,

    /// WINDOWS: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[cfg_attr(feature = "clap", clap(long = "command-windows"))]
    #[serde(skip)]
    pub windows_command: Option<String>,

    /// MACOS: Command to run before spawing a window, useful for starting web servers for WebApps.
    #[cfg_attr(feature = "clap", clap(long = "command-macos"))]
    #[serde(skip)]
    pub macos_command: Option<String>,

    #[cfg_attr(feature = "clap", clap(skip))]
    /// Command to run before spawning a window, useful for starting web servers for WebApps.
    pub command: Option<String>,

    /// LINUX: If specified a .desktop entry will NOT be generated.
    ///
    /// Entries are saved on "~/.local/share/applications" by default.
    /// Set `desktop-entry-path` to change this.
    #[cfg_attr(feature = "clap", clap(long = "no-desktop"))]
    #[serde(skip)]
    pub no_desktop: bool,

    /// LINUX: Path where the .desktop entry will be generated.
    #[cfg_attr(feature = "clap", clap(long = "desktop-entry-path", default_value_t = String::from("~/.local/share/applications")))]
    #[serde(skip)]
    pub desktop_entry_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
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
