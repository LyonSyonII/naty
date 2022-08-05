#![windows_subsystem = "windows"]

fn main() -> std::io::Result<()> {
    // If on Application mode, run WebView
    if let Ok(file) = std::fs::File::open("naty.toml") {
        naty_app::run(file).unwrap();
    } else {
        naty_nativefy::run()?;
    }

    Ok(())
}
