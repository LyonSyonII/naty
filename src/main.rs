mod application;
mod nativefy;

fn main() -> wry::Result<()> {
    // If on Application mode, run WebView
    if let Ok(file) = std::fs::File::open("naty.toml") {
        application::run(file)?;
    } else {
        nativefy::run()?;
    }

    Ok(())
}
