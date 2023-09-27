use std::path::PathBuf;

use crate::snapshot::{Comparison, Info};

#[derive(PartialEq)]
pub enum Mode {
    ScreenMode,
    FileMode,
}

pub fn compare_snapshots<T: Comparison>(
    snap_1: &T,
    snap_2: &T,
    path: &mut PathBuf,
    output_fn: fn(),
    mode: &Mode,
    result_file: &str,
) {
    compare_size(
        snap_1.get_size(),
        snap_2.get_size(),
        "snapshot",
        &mode,
        result_file,
    );

    for file in snap_1.get_files() {
        let index = find_item(&file.name, snap_2.get_files());
        if index != -1 {
            let new_file = &snap_2.get_files()[index as usize];
            compare_size(
                file.size_kb,
                new_file.size_kb,
                &file.name,
                &mode,
                result_file,
            );
        } else {
            let mut new_path = PathBuf::from(path.to_str().unwrap().trim());
            new_path.push(&file.name);
            print_file_del(&mode, &new_path.to_str().unwrap(), result_file);
        }
    }

    for dir in snap_1.get_dirs() {
        let index = find_item(&dir.name, snap_2.get_dirs());
        if index != -1 {
            let new_dir = &snap_2.get_dirs()[index as usize];
            compare_size(dir.size_kb, new_dir.size_kb, &dir.name, &mode, result_file);
            path.push(dir.get_name());
            compare_snapshots(dir, new_dir, path, output_fn, &mode, result_file);
        } else {
            let mut new_path = PathBuf::from(path.to_str().unwrap().trim());
            new_path.push(&dir.name);
            print_dir_del(&mode, &new_path.to_str().unwrap(), result_file);
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

#[cfg(not(tarpaulin_include))]
pub fn compare_size(size_1: u64, size_2: u64, item_name: &str, mode: &Mode, result_file: &str) {
    use crate::common::append_to_file;

    match mode {
        Mode::ScreenMode => {
            if size_1 != size_2 {
                println!("{item_name} was changed (size: {size_1}kb -> {size_2}kb)");
            }
        }
        Mode::FileMode => {
            if size_1 != size_2 {
                append_to_file(
                    result_file,
                    &format!("{item_name} was changed (size: {size_1}kb -> {size_2}kb)"),
                )
                .expect("File Error");
            }
        }
    };
}

#[cfg(not(tarpaulin_include))]
pub fn print_file_del(mode: &Mode, path: &str, result_file: &str) {
    use crate::common::append_to_file;

    match mode {
        Mode::ScreenMode => println!("File \"{path}\" was deleted"),
        Mode::FileMode => {
            append_to_file(result_file, &format!("File \"{path}\" was deleted"))
                .expect("File Error");
        }
    };
}

#[cfg(not(tarpaulin_include))]
pub fn print_dir_del(mode: &Mode, path: &str, result_file: &str) {
    use crate::common::append_to_file;

    match mode {
        Mode::ScreenMode => println!("Directory \"{path}\" was deleted"),
        Mode::FileMode => {
            append_to_file(result_file, &format!("Directory \"{path}\" was deleted"))
                .expect("File Error");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{commands::snapshot_gallery::snapshot_comparison::find_item, snapshot::File};

    fn create_file(name: &str) -> File {
        File {
            name: String::from(name),
            size_kb: 10,
        }
    }

    #[test]
    fn test_find_item_1() {
        let files = Vec::from([
            create_file("first"),
            create_file("second"),
            create_file("third"),
        ]);
        let result = find_item("first", &files);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_find_item_2() {
        let files = Vec::from([
            create_file("first"),
            create_file("second"),
            create_file("third"),
        ]);
        let result = find_item("second", &files);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_find_non_existent_item() {
        let files = Vec::from([
            create_file("first"),
            create_file("second"),
            create_file("third"),
        ]);
        let result = find_item("fourth", &files);
        assert_eq!(result, -1);
    }
}
