use std::env;
use std::path::Path;

use crate::errors::ServiceError;
use crate::snapshot::{self, Snapshot};

use chrono::Local;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::Database;

pub async fn create_snapshot(path: &Path, db: &Database) -> Result<(), ServiceError> {
    let collection_name = &env::var("COLL_NAME").expect("COLL_NAME must be set");
    let snapshot_path = String::from(path.to_str().unwrap());
    let collection: Collection<Snapshot> = db.collection(&collection_name);

    let version = get_version(&collection, &snapshot_path)
        .await
        .expect("Version Error");
    let date = Local::now().to_string();

    let mut snapshot = snapshot::Snapshot::create(version, date, snapshot_path);

    let size = snapshot::fill_and_return_size(path, &mut snapshot.files, &mut snapshot.dirs);
    snapshot.set_size(size);

    match db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"))
        .insert_one(snapshot, None)
        .await {
            Ok(res) => res,
            Err(_) => return Err(ServiceError::FailedToFoundCollection)
        };

    Ok(())
}

pub async fn get_version(
    collection: &Collection<Snapshot>,
    path: &str,
) -> Result<i32, ServiceError> {
    let mut cursor = match collection.find(doc! {"path": path}, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err(ServiceError::FailedToFoundCollection)
    };
    let mut version = 0;

    while cursor.advance().await.unwrap() {
        version += 1
    }
    Ok(version + 1)
}
