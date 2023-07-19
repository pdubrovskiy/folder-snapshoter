use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::common::get_size;

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: i32,
    date: String,
    size: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    name: String,
    size: u64,
    files: Vec<File>,
    dirs: Vec<Directory>,
}

impl Directory {}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    size: u64,
}

impl Snapshot {
    pub fn create(version: i32, date: String) -> Snapshot {
        Snapshot {
            version: version,
            date: date,
            size: 0,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

pub fn fill_and_return_size(path: &Path, files: &mut Vec<File>, dirs: &mut Vec<Directory>) -> u64 {
    for item in path.read_dir().expect("Reading dir was failed") {
        if let Ok(item) = item {
            let item_name = String::from(
                item.path()
                    .file_name()
                    .expect("File Error")
                    .to_str()
                    .expect("File name Error"),
            );

            if item.path().is_file() {
                let file = File {
                    name: item_name.clone(),
                    size: get_size(&item.path()),
                };
                files.push(file);
            } else {
                let mut dir = Directory {
                    name: item_name.clone(),
                    size: 0,
                    files: Vec::new(),
                    dirs: Vec::new(),
                };
                let mut new_path = PathBuf::from(path);
                new_path.push(&item_name);
                dir.size = fill_and_return_size(&new_path, files, dirs);
                dirs.push(dir);
            }
        }
    }

    files.iter().fold(0, |s, e| s + e.size) + dirs.iter().fold(0, |s, e| s + e.size)
}
