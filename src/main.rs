use std::path::Path;
use wry::{application::{self, window::{WindowBuilder, Window}, window::Icon, event_loop::ControlFlow, event::{StartCause, Event, WindowEvent}}, webview::WebViewBuilder};

#[derive(serde::Deserialize, Debug)]
struct AppSettings<'i> {
    url: &'i str,
    title: &'i str,
    icon: bool,
}


fn copy_executable() {

}

fn run_application(file: std::fs::File) -> wry::Result<()> {
    let settings: AppSettings = toml::from_str(include_str!("../naty.toml")).unwrap();
    let event_loop = application::event_loop::EventLoop::new();
    let icon = image::load_from_memory(include_bytes!("../icon.png")).unwrap();
    let width = icon.width();
    let height = icon.height();
    
    let window = WindowBuilder::new().with_title(settings.title).build(&event_loop)?;
        
    if settings.icon {
        window.set_window_icon(Some(Icon::from_rgba(icon.into_bytes(), width, height)?));
    }
    
    let _webview = WebViewBuilder::new(window)?
        .with_url(settings.url)?
        .build()?;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit
        }
    });
}

fn run_nativefier() {

}

fn main() -> wry::Result<()> {
    // If on Application mode, run WebView
    if let Ok(file) = std::fs::File::open("naty.toml") {
        run_application(file)?;
    } else {
        run_nativefier();
    }

    Ok(())
}
