use std::{
    io,
    path::{Path, PathBuf},
};

use crate::common;

pub fn nav_menu(path: &mut PathBuf) {
    loop {
        println!("|______NAV_MODE______|");
        println!("1. Go up");
        println!("2. Go to path");
        println!("3. Show directory content");
        println!("4. Show current path");
        println!("ENTER ANOTHER NUMBER TO EXIT FROM NAV MODE!");

        match common::get_input() {
            1 => {
                up(path);
            }
            2 => {
                cd(path);
            }
            3 => {
                ls(path);
            }
            4 => {
                pwd(path);
            }
            _ => break,
        }
    }
}

pub fn up(path: &mut PathBuf) {
    path.pop();
}

pub fn cd(path: &mut PathBuf) {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Path Error");

    let input = PathBuf::from(input.trim());

    if input.is_relative() {
        path.push(input);

        if !path.exists() {
            path.pop();
            println!("Sorry but your path isn't correct");
        }
    } else {
        if input.exists() {
            path.clear();
            input.iter().for_each(|e| path.push(e));
        } else {
            println!("Sorry but your path isn't correct");
        }
    }
}

pub fn ls(path: &Path) {
    for entry in path.read_dir().expect("Reading dir was failed") {
        if let Ok(entry) = entry {
            let p: std::path::PathBuf = entry.path();
            let t = if p.is_dir() { "directory" } else { "file" };
            println!("|{:?}| {}", p.file_name().unwrap(), t);
        }
    }
}

pub fn pwd(path: &PathBuf) {
    println!("Current path: \n{:?}", path);
}
