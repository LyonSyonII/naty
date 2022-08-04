use std::io::Read;

use wry::{application::{self, window::{WindowBuilder, Window}, window::Icon, event_loop::ControlFlow, event::{StartCause, Event, WindowEvent}}, webview::WebViewBuilder};

#[derive(serde::Deserialize, Debug)]
struct AppSettings<'i> {
    url: &'i str,
    title: &'i str,
    icon: bool,
}

fn exit(msg: impl AsRef<str>) -> ! {
    let msg = msg.as_ref();
    eprintln!("{msg}");
    std::process::exit(1)
}

pub fn run(mut file: std::fs::File) -> wry::Result<()> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    drop(file);

    let settings: AppSettings = toml::from_str(&buffer).unwrap();
    let event_loop = application::event_loop::EventLoop::new();
    
    let window = WindowBuilder::new().with_title(settings.title).build(&event_loop)?;
        
    if settings.icon {
        let icon = std::fs::read("icon.png").unwrap_or_exit("icon.png is not present in the root directory.\nSet icon=false on naty.toml or add an icon");

        let icon = image::load_from_memory(&icon).unwrap();
        let width = icon.width();
        let height = icon.height();
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



trait Exit<T, E> {
    fn unwrap_or_exit(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> Exit<T, E> for Result<T, E> {
    fn unwrap_or_exit(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| {
            eprintln!("{}", msg.as_ref());
            std::process::exit(1)
        })
    }
}