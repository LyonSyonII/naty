use std::io::Read;

use wry::{
    application::{
        self,
        dpi::{PhysicalSize, Size},
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
        menu::MenuBar,
        window::Icon,
        window::{self, WindowBuilder}
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
    
    let url: wry::webview::Url = settings.target_url.as_str().try_into().unwrap_or_else(|err| {
        println!("Error parsing the url: {err}");
        std::process::exit(1)
    });
    
    let mut command = settings.command.and_then(|cmd| {
        let mut cmd = cmd.split_whitespace();
        std::process::Command::new(cmd.next()?).args(cmd.collect::<Vec<_>>()).spawn().ok()
    });

    let name = naty_common::get_webpage_name(settings.name.as_deref(), &url);
    let window = 
        WindowBuilder::new()
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
        .build(&event_loop)?;
    window.set_menu(settings.show_menu_bar.then_some(MenuBar::new()));
    
    // Add icon to executable
    if let Ok(icon) = std::fs::read(naty_common::get_exe_path().join("icon.png")) {
        let icon = image::load_from_memory(&icon).unwrap();
        let width = icon.width();
        let height = icon.height();
        window.set_window_icon(Some(Icon::from_rgba(icon.into_bytes(), width, height)?));
    } else {
        eprintln!("Icon not found in executable folder, none will be used.")
    }

    let _webview = WebViewBuilder::new(window)?
        .with_url(&settings.target_url)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
            // If application exited, kill command
            if let Some(command) = &mut command {
                command.kill();
            }
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
