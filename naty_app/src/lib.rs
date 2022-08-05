use std::io::Read;

use wry::{
    application::{
        self,
        dpi::{PhysicalSize, Size},
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
        menu::MenuBar,
        window::Icon,
        window::{self, WindowBuilder},
    },
    webview::WebViewBuilder,
};

use naty_common::AppSettings;

/// Runs the application based on the provided `naty.toml`
///
/// Returns the resulting `wry::Result<()>` from the WebView invocation.  
/// 
/// See [`Wry`](https://github.com/tauri-apps/wry) for more information
/// 
/// # Example
/// 
/// 
pub fn run(mut file: std::fs::File) -> wry::Result<()> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    drop(file);
    
    let settings: AppSettings = toml::from_str(&buffer).unwrap();
    let event_loop = application::event_loop::EventLoop::new();
    
    let name = naty_common::get_webpage_name(settings.name.as_ref(), &settings.target_url);
    let window = WindowBuilder::new()
        .with_title(name)
        .with_always_on_top(settings.always_on_top)
        .with_fullscreen(
            settings
                .full_screen
                .then_some(window::Fullscreen::Borderless(None)),
        )
        .with_inner_size(Size::Physical(PhysicalSize::new(
            settings.width,
            settings.height,
        )))
        .with_max_inner_size(Size::Physical(PhysicalSize::new(
            settings.max_width,
            settings.max_height,
        )))
        .with_min_inner_size(Size::Physical(PhysicalSize::new(
            settings.min_width,
            settings.min_height,
        )))
        .with_decorations(!settings.hide_window_frame)
        .with_menu(MenuBar::new())
        .build(&event_loop)?;

    window.set_menu(settings.show_menu_bar.then_some(MenuBar::new()));

    if let Some(path) = settings.icon {
        let icon = std::fs::read(naty_common::get_exe_dir().join(&path)).unwrap_or_exit(format!("Icon '{}' does not exist. Please change the 'icon' option in 'naty.toml' or remove it completely.", path.display()));
        
        let icon = image::load_from_memory(&icon).unwrap();
        let width = icon.width();
        let height = icon.height();
        window.set_window_icon(Some(Icon::from_rgba(icon.into_bytes(), width, height)?));
    }

    let _webview = WebViewBuilder::new(window)?
        .with_url(&settings.target_url)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}

trait Exit<T> {
    fn unwrap_or_exit(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> Exit<T> for Result<T, E> {
    fn unwrap_or_exit(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| {
            eprintln!("{}", msg.as_ref());
            std::process::exit(1)
        })
    }
}

impl<T> Exit<T> for Option<T> {
    fn unwrap_or_exit(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|| {
            eprintln!("{}", msg.as_ref());
            std::process::exit(1)
        })
    }
}
