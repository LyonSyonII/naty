mod application;
mod nativefy;


fn copy_executable() {

}


fn run_nativefier() {

}

fn main() -> wry::Result<()> {
    // If on Application mode, run WebView
    if let Ok(file) = std::fs::File::open("naty.toml") {
        application::run(file)?;
    } else {
        run_nativefier();
    }

    Ok(())
}
