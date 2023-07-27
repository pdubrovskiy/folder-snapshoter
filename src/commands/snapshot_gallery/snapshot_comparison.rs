use std::path::PathBuf;

use crate::snapshot::{Comparison, Info};

pub fn compare_snapshots<T: Comparison>(snap_1: &T, snap_2: &T, path: &mut PathBuf) {
    compare_size(snap_1.get_size(), snap_2.get_size(), "snapshot");

    for file in snap_1.get_files() {
        let index = find_item(&file.name, snap_2.get_files());
        if index != -1 {
            let new_file = &snap_2.get_files()[index as usize];
            compare_size(file.size_kb, new_file.size_kb, &file.name);
        } else {
            println!(
                "File \"{}/{}\" was deleted",
                path.to_str().unwrap().trim(),
                file.name
            );
        }
    }

    for dir in snap_1.get_dirs() {
        let index = find_item(&dir.name, snap_2.get_dirs());
        if index != -1 {
            let new_dir = &snap_2.get_dirs()[index as usize];
            compare_size(dir.size_kb, new_dir.size_kb, &dir.name);
            path.push(dir.get_name());
            compare_snapshots(dir, new_dir, path);
        } else {
            println!(
                "Directory \"{}/{}\" was deleted",
                path.to_str().unwrap().trim(),
                dir.name
            );
        }
    }
}

pub fn find_item<T: Info>(item_name: &str, items: &Vec<T>) -> i32 {
    for (i, item) in items.iter().enumerate() {
        if item.get_name() == item_name {
            return i as i32;
        }
    }
    -1
}

pub fn compare_size(size_1: u64, size_2: u64, item_name: &str) {
    if size_1 != size_2 {
        println!("{item_name} was changed (size: {size_1}kb -> {size_2}kb)");
    }
}
