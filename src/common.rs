use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub fn get_input() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Input Error");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Incorrect input. Please repeat your attempt");
                continue;
            }
        };
    }
}

pub fn get_initial_path(str: &str) -> PathBuf {
    let mut path = PathBuf::from(str);
    path.pop();
    path
}

pub fn get_size(path: &Path) -> u64 {
    let f = File::open(path).expect("File Error");
    let size = f.metadata().unwrap().len();

    size
}
