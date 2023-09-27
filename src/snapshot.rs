use std::path::{Path, PathBuf};

use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{common::get_size, errors::ServiceError};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub version: i32,
    pub date: String,
    pub path: String,
    pub size_kb: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Directory {
    pub name: String,
    pub size_kb: u64,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

impl Directory {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {

    use std::{env, fs, path::PathBuf};

    use mongodb::Collection;

    use crate::{
        db,
        snapshot::{fill_and_return_size, get_version, Comparison, File, Info, Snapshot},
    };

    use super::Directory;

    fn create_file() -> File {
        File {
            name: String::from("test"),
            size_kb: 10,
        }
    }

    fn create_dir() -> Directory {
        Directory {
            name: String::from("test"),
            size_kb: 20,
            files: Vec::from([create_file()]),
            dirs: Vec::new(),
        }
    }

    fn create_snapshot() -> Snapshot {
        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();
        path.push("snapshot");

        Snapshot {
            version: 1,
            date: String::from("date"),
            path: String::from(path.to_str().unwrap()),
            size_kb: 10,
            files: Vec::from([create_file(), create_file(), create_file()]),
            dirs: Vec::from([create_dir(), create_dir(), create_dir(), create_dir()]),
        }
    }

    fn set_env_variables() {
        env::set_var("DB_NAME", "snapshots_gallery");
        env::set_var(
            "DB_URI",
            "mongodb+srv://user:user@cluster0.ycjzrmp.mongodb.net/?retryWrites=true&w=majority",
        );
        env::set_var("COLL_NAME", "snapshots");
    }

    #[tokio::test]
    async fn test_get_version_of_non_existent_snapshot() {
        set_env_variables();

        let path = "non-existent-path";
        let db = db::connect_db().await.unwrap();
        let collection: Collection<Snapshot> =
            db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"));

        let result = get_version(&collection, path).await.unwrap();
        assert_eq!(result, 0);
    }
    #[tokio::test]
    async fn test_get_version_of_existent_snapshot_1() {
        set_env_variables();

        let path = "test1";
        let db = db::connect_db().await.unwrap();
        let collection: Collection<Snapshot> =
            db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"));

        let result = get_version(&collection, path).await.unwrap();
        assert_eq!(result, 1);
    }
    #[tokio::test]
    async fn test_get_version_of_existent_snapshot_2() {
        set_env_variables();

        let path = "test2";
        let db = db::connect_db().await.unwrap();
        let collection: Collection<Snapshot> =
            db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"));

        let result = get_version(&collection, path).await.unwrap();
        assert_eq!(result, 2);
    }
    #[test]
    fn test_create_snapshot() {
        let snapshot = Snapshot {
            version: 1,
            date: String::new(),
            path: String::new(),
            size_kb: 0,
            files: Vec::new(),
            dirs: Vec::new(),
        };
        assert_eq!(Snapshot::create(1, String::new(), String::new()), snapshot);
    }

    #[test]
    fn test_set_size_snapshot() {
        let mut snapshot = create_snapshot();
        snapshot.set_size(50);
        assert_eq!(snapshot.size_kb, 50);
    }

    #[test]
    fn test_get_size_snapshot() {
        let snapshot = create_snapshot();
        assert_eq!(snapshot.get_size(), 10);
    }
    #[test]
    fn test_get_files_snapshot() {
        let snapshot = create_snapshot();
        let test_value: Vec<File> = Vec::from([create_file(), create_file(), create_file()]);
        assert_eq!(snapshot.get_files(), &test_value);
    }
    #[test]
    fn test_get_dirs_snapshot() {
        let snapshot = create_snapshot();
        let test_value: Vec<Directory> =
            Vec::from([create_dir(), create_dir(), create_dir(), create_dir()]);
        assert_eq!(snapshot.get_dirs(), &test_value);
    }
    #[test]
    fn test_get_size_directory() {
        let dir = create_dir();
        assert_eq!(dir.get_size(), 20);
    }
    #[test]
    fn test_get_files_directory() {
        let dir = create_dir();
        let test_value: Vec<File> = Vec::from([create_file()]);

        assert_eq!(dir.get_files(), &test_value);
    }
    #[test]
    fn test_get_dirs_directory() {
        let dir = create_dir();

        let test_value: Vec<Directory> = Vec::new();
        assert_eq!(dir.get_dirs(), &test_value);
    }
    #[test]
    fn test_get_name_directory() {
        let dir = create_dir();
        assert_eq!(dir.get_name(), String::from("test"));
    }
    #[test]
    fn test_get_name_file() {
        let file = create_file();
        assert_eq!(file.get_name(), String::from("test"));
    }
    #[test]
    fn test_fill_and_return_size() {
        let mut snapshot = create_snapshot();
        fs::create_dir(&snapshot.path).unwrap();

        for num in 0..10 {
            let mut file_path = PathBuf::from(&snapshot.path);
            file_path.push(num.to_string());
            fs::write(&file_path, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").unwrap();
        }

        for num in 0..5 {
            let mut dir_path = PathBuf::from(&snapshot.path);
            let mut dir_name = String::from("dir");
            dir_name.push_str(&num.to_string());
            dir_path.push(dir_name);
            fs::create_dir(&dir_path).unwrap();
        }

        let size = fill_and_return_size(
            &PathBuf::from(&snapshot.path),
            &mut snapshot.files,
            &mut snapshot.dirs,
        );

        fs::remove_dir_all(&snapshot.path).unwrap();
        assert_eq!(size, 4);
    }
}
