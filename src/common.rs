use std::fs::{File, OpenOptions};
use std::io::{self, Write};
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

pub fn append_to_file(file_path: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    file.write_all(data.as_bytes())?;
    file.write("\n".as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use crate::common::get_size;

    use super::get_initial_path;

    #[test]
    fn test_get_initial_path() {
        let result = get_initial_path("home/test/test.txt");
        assert_eq!(result, PathBuf::from("home/test"));
    }
    #[test]
    fn test_get_size() {
        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();
        path.push("test_file");
        fs::write(
            &path,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        )
        .unwrap();
        let result = get_size(&path);
        assert_eq!(result, 55);
    }
}
