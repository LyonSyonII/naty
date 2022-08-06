// #![windows_subsystem = "windows"]

fn main() -> std::io::Result<()> {
    // If on Application mode, run WebView
    if let Ok(file) = std::fs::File::open(naty_common::get_exe_path().join("naty.toml")) {
        naty_app::run(file).unwrap();
    } 
    // If running from Cli, create app
    else {
        naty_nativefy::run()?;
    }
    
    Ok(())
}
