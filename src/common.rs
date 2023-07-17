use std::{io, path::PathBuf};

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

pub fn get_initial_path(str: &str) -> PathBuf{
    let mut path = PathBuf::from(str);
    path.pop();
    path
}