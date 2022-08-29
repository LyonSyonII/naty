mod lib;

fn main() {
    let exe_path = naty_common::get_exe_path();
    match std::fs::File::open(exe_path.join("naty.toml")) {
        Ok(file) => lib::run(file).unwrap(),
        Err(_) => eprintln!("error: no 'naty.toml' file in {}", exe_path.display()),
    }
}