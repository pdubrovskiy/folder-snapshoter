use std::env;
use std::path::PathBuf;

use crate::common;
use crate::errors::ServiceError;
use crate::snapshot::{self, Snapshot};
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::Database;

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
                let mut version = common::get_input();

                if version == 0 {
                    version = snapshot::get_version(&collection, path).await?;

                    if version == 0 {
                        println!("Versions not found");
                        break;
                    }
                }
                show_version(path, &collection, version).await?;
            }
            3 => {

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
    let found_item = match collection
        .find_one(doc! {"path": path, "version": version}, None)
        .await
    {
        Ok(item) => item,
        Err(_) => return Err(ServiceError::FailedToFoundCollection),
    };

    match found_item {
        Some(snapshot) => println!("{:#?}", snapshot),
        None => println!("Snapshot isn't found. Check the version"),
    }

    Ok(())
}
