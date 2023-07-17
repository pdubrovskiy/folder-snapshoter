use std::path::PathBuf;

mod exit;
mod navigation;

pub fn run_command(input: i32, path: &mut PathBuf) {
    match input {
        1 => {
            navigation::nav_menu(path);
        }
        5 => {
            exit::exit();
        }
        _ => {
            println!("Incorrect input. Please repeat your attempt");
        }
    };
}
