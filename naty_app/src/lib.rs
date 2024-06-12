use std::io::Read;

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

    let mut command = settings.command.and_then(|cmd| {
        let mut cmd = cmd.split_whitespace();
        std::process::Command::new(cmd.next()?)
            .args(cmd.collect::<Vec<_>>())
            .spawn()
            .ok()
    });

    let name = naty_common::get_webpage_name(settings.name.as_deref(), &settings.target_url);

    let event_loop = tao::event_loop::EventLoop::new();
    let window = tao::window::WindowBuilder::new()
        .with_title(name)
        .with_always_on_top(settings.always_on_top)
        .with_fullscreen(
            settings
                .full_screen
                .then_some(tao::window::Fullscreen::Borderless(None)),
        )
        .with_inner_size(
            tao::dpi::PhysicalSize::new(settings.width, settings.height),
        )
        .with_inner_size_constraints(tao::window::WindowSizeConstraints::new(
            settings.min_width.map(|n| n as i32).map(tao::dpi::PhysicalUnit::new).map(tao::dpi::PixelUnit::Physical),
            settings.min_height.map(|n| n as i32).map(tao::dpi::PhysicalUnit::new).map(tao::dpi::PixelUnit::Physical),
            settings.max_width.map(|n| n as i32).map(tao::dpi::PhysicalUnit::new).map(tao::dpi::PixelUnit::Physical),
            settings.max_height.map(|n| n as i32).map(tao::dpi::PhysicalUnit::new).map(tao::dpi::PixelUnit::Physical),
        ))
        .with_decorations(!settings.hide_window_frame)
        .build(&event_loop)
        .unwrap();

    // Add icon to executable
    if let Ok(icon) = std::fs::read(naty_common::get_exe_path().join("icon.png")) {
        let icon = image::load_from_memory(&icon).unwrap();
        let width = icon.width();
        let height = icon.height();
        window.set_window_icon(Some(tao::window::Icon::from_rgba(icon.into_bytes(), width, height).unwrap()));
    } else {
        eprintln!("Icon not found in executable folder, none will be used.")
    }
        
    let _webview = {
        #[cfg(not(target_os = "linux"))] {
            wry::WebViewBuilder::new(&window)
        }
        #[cfg(target_os = "linux")] {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            wry::WebViewBuilder::new_gtk(window.gtk_window())
        }
    }.with_url(&settings.target_url)
    .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Poll;
        
        if let tao::event::Event::WindowEvent {
            event: tao::event::WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = tao::event_loop::ControlFlow::Exit;
            // If application exited, kill command
            if let Some(command) = command.as_mut() {
                let res = command.kill();
                match res {
                    Ok(_) => {}
                    Err(e) => println!("error: {e}"),
                }
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
