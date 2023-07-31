use std::{
    io,
    path::{Path, PathBuf},
};

use crate::common;

pub fn nav_menu(path: &mut PathBuf) {
    loop {
        match common::get_input() {
            1 => {
                up(path);
            }
            2 => {
                let mut input = String::new();

                println!("Please input absolute or relative path");
                io::stdin().read_line(&mut input).expect("Path Error");

                cd(path, input);
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

#[cfg(not(tarpaulin_include))]
fn print_nav_menu() {
    println!("|______NAV_MODE______|");
    println!("1. Go up");
    println!("2. Go to path");
    println!("3. Show directory content");
    println!("4. Show current path");
    println!("ENTER ANOTHER NUMBER TO EXIT FROM NAV MODE!");
}

pub fn up(path: &mut PathBuf) {
    path.pop();
}

pub fn cd(path: &mut PathBuf, input: String) {
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

#[cfg(not(tarpaulin_include))]
pub fn ls(path: &Path) {
    for entry in path.read_dir().expect("Reading dir was failed") {
        if let Ok(entry) = entry {
            let p: std::path::PathBuf = entry.path();
            let t = if p.is_dir() { "directory" } else { "file" };
            println!("|{:?}| {}", p.file_name().unwrap(), t);
        }
    }
}

#[cfg(not(tarpaulin_include))]
pub fn pwd(path: &PathBuf) {
    println!("Current path: \n{:?}", path);
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use super::{cd, up};

    #[test]
    fn test_up() {
        let mut result = PathBuf::from("home/test");
        up(&mut result);
        assert_eq!(result, PathBuf::from("home"));
    }
    #[test]
    fn test_cd_with_relative_path() {
        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();

        path.push("test");
        fs::create_dir(&path).unwrap();
        let test_value = path.clone();
        path.pop();

        cd(&mut path, String::from("test"));

        fs::remove_dir(&test_value).unwrap();

        assert_eq!(path, test_value);
    }

    #[test]
    fn test_cd_with_non_existent_relative_path() {
        let mut path = PathBuf::from("non-existent-path");
        let test_value = path.clone();

        cd(&mut path, String::from("test"));

        assert_eq!(path, test_value);
    }
    #[test]
    fn test_cd_with_absolute_path() {
        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();

        let mut test_value = path.clone();
        test_value.pop();

        cd(&mut path, String::from(test_value.to_str().unwrap()));

        assert_eq!(path, test_value);
    }

    #[test]
    fn test_cd_with_non_existent_absolute_path() {
        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();

        let mut test_value = path.clone();
        test_value.push("non-existent-path");

        cd(&mut path, String::from(test_value.to_str().unwrap()));

        test_value.pop();

        assert_eq!(path, test_value);
    }
}
