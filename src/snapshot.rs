use std::path::{Path, PathBuf};

use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{common::get_size, errors::ServiceError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: i32,
    date: String,
    pub path: String,
    pub size_kb: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub size_kb: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

impl Directory {}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub size_kb: u64,
}

pub trait Comparison {
    fn get_size(&self) -> u64;
    fn get_files(&self) -> &Vec<File>;
    fn get_dirs(&self) -> &Vec<Directory>;
}

impl Comparison for Snapshot {
    fn get_size(&self) -> u64 {
        self.size_kb
    }
    fn get_files(&self) -> &Vec<File> {
        &self.files
    }
    fn get_dirs(&self) -> &Vec<Directory> {
        &self.dirs
    }
}

impl Comparison for Directory {
    fn get_size(&self) -> u64 {
        self.size_kb
    }
    fn get_files(&self) -> &Vec<File> {
        &self.files
    }
    fn get_dirs(&self) -> &Vec<Directory> {
        &self.dirs
    }
}

pub trait Info {
    fn get_name(&self) -> &str;
}

impl Info for Directory {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Info for File {
    fn get_name(&self) -> &str {
        &self.name
    }
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
    println!("----");
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
                dir.size_kb = fill_and_return_size(&new_path, &mut dir.files, &mut dir.dirs);
                dirs.push(dir);
            }
        }
    }

    (files.iter().fold(0, |s, e| s + e.size_kb) + dirs.iter().fold(0, |s, e| s + e.size_kb)) / 1024
}

pub async fn get_version(
    collection: &Collection<Snapshot>,
    path: &str,
) -> Result<i32, ServiceError> {
    let mut cursor = match collection.find(doc! {"path": path}, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err(ServiceError::FailedToFoundCollection),
    };
    let mut version = 0;

    while cursor.advance().await.unwrap() {
        version += 1
    }
    Ok(version)
}
