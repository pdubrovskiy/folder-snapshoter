use std::env;
use std::path::PathBuf;

use crate::common;
use crate::errors::ServiceError;
use crate::snapshot::{self, Snapshot};
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::Database;

mod snapshot_comparison;

pub async fn snapshot_gallery_menu(path: &PathBuf, db: &Database) -> Result<(), ServiceError> {
    let path = path.to_str().unwrap();
    let collection: Collection<Snapshot> =
        db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"));

    loop {
        println!("|____SNAPSHOT_GALLERY____|");
        println!("1. Show list of versions");
        println!("2. Show the version");
        println!("3. Compare snapshots");
        println!("ENTER ANOTHER NUMBER TO EXIT FROM SNAPSHOT GALLERY!");

        match common::get_input() {
            1 => {
                show_list_of_versions(path, &collection).await?;
            }
            2 => {
                println!("Please, input the number of version or 0 to choose the last one:");
                let version = common::get_input();
                show_version(path, &collection, version).await?;
            }
            3 => {
                println!("Please input the version of snapshot to compare or 0 - to choose the last one: ");
                let version_1 = common::get_input();

                println!("Please input the version of snapshot to compare or 0 - to choose the last one: ");
                let version_2 = common::get_input();

                let last_version = snapshot::get_version(&collection, path).await?;

                if version_1 == version_2
                    || version_1 == 0 && version_2 == last_version
                    || version_1 == last_version && version_2 == 0
                {
                    println!("Sorry, you chose the same snapshots");
                    continue;
                }

                compare(path, &collection, version_1, version_2).await?;
            }
            _ => break,
        }
    }
    Ok(())
}

pub async fn show_list_of_versions(
    path: &str,
    collection: &Collection<Snapshot>,
) -> Result<(), ServiceError> {
    let mut cursor = match collection.find(doc! {"path": path}, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err(ServiceError::FailedToFoundCollection),
    };

    while cursor.advance().await.unwrap() {
        println!(
            "version: {:?}",
            cursor
                .current()
                .get("version")
                .unwrap()
                .expect("There isn't version")
                .as_i32()
                .unwrap()
        );
    }

    Ok(())
}

pub async fn show_version(
    path: &str,
    collection: &Collection<Snapshot>,
    version: i32,
) -> Result<(), ServiceError> {
    let snapshot = get_snapshot(path, collection, version).await?;

    if snapshot.is_none() {
        println!("Snapshot isn't found. Check the version");
    } else {
        println!("{:#?}", snapshot);
    }

    Ok(())
}

pub async fn get_snapshot(
    path: &str,
    collection: &Collection<Snapshot>,
    version: i32,
) -> Result<Option<Snapshot>, ServiceError> {
    let mut v = version;
    if v == 0 {
        v = snapshot::get_version(&collection, path).await?;
    }
    let found_item = match collection
        .find_one(doc! {"path": path, "version": v}, None)
        .await
    {
        Ok(item) => item,
        Err(_) => return Err(ServiceError::FailedToFoundCollection),
    };

    Ok(found_item)
}

pub async fn compare(
    path: &str,
    collection: &Collection<Snapshot>,
    v1: i32,
    v2: i32,
) -> Result<(), ServiceError> {
    let snapshot_1 = get_snapshot(path, &collection, v1).await?;
    let snapshot_2 = get_snapshot(path, &collection, v2).await?;

    if snapshot_1.is_none() || snapshot_2.is_none() {
        println!("Sorry, but we can't find snapshot. Please check versions");
    } else {
        let snapshot_1 = snapshot_1.unwrap();
        let snapshot_2 = snapshot_2.unwrap();
        let mut path = PathBuf::new();
        snapshot_comparison::compare_snapshots(&snapshot_1, &snapshot_2, &mut path);
    };

    Ok(())
}
