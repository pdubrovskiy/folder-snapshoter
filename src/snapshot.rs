use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::common::get_size;

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: i32,
    date: String,
    path: String,
    size_kb: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    name: String,
    size_kb: u64,
    files: Vec<File>,
    dirs: Vec<Directory>,
}

impl Directory {}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    size_kb: u64,
}

impl Snapshot {
    pub fn create(version: i32, date: String, path: String) -> Snapshot {
        Snapshot {
            version: version,
            date: date,
            path: path,
            size_kb: 0,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
    pub fn set_size(&mut self, size: u64) {
        self.size_kb = size;
    }
}

pub fn fill_and_return_size(path: &Path, files: &mut Vec<File>, dirs: &mut Vec<Directory>) -> u64 {
    for entry in path.read_dir().expect("Reading dir was failed") {
        if let Ok(entry) = entry {
            let item_name = String::from(
                entry
                    .path()
                    .file_name()
                    .expect("File Error")
                    .to_str()
                    .expect("File name Error"),
            );

            if entry.path().is_file() {
                let file = File {
                    name: item_name.clone(),
                    size_kb: get_size(&entry.path()),
                };
                files.push(file);
            } else {
                let mut dir = Directory {
                    name: item_name.clone(),
                    size_kb: 0,
                    files: Vec::new(),
                    dirs: Vec::new(),
                };
                let mut new_path = PathBuf::from(path);
                new_path.push(&item_name);
                dir.size_kb = fill_and_return_size(&new_path, files, dirs);
                dirs.push(dir);
            }
        }
    }

    (files.iter().fold(0, |s, e| s + e.size_kb) + dirs.iter().fold(0, |s, e| s + e.size_kb)) / 1024
}
