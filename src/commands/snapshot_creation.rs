use std::env;
use std::path::Path;

use crate::snapshot::{self, Snapshot};

use chrono::Local;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::Database;

pub async fn create_snapshot(path: &Path, db: &Database) -> mongodb::error::Result<()> {
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

    db.collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"))
        .insert_one(snapshot, None)
        .await?;

    Ok(())
}

pub async fn get_version(
    collection: &Collection<Snapshot>,
    path: &str,
) -> mongodb::error::Result<i32> {
    let mut cursor = collection.find(doc! {"path": path}, None).await?;
    let mut version = 0;

    while cursor.advance().await? {
        version += 1
    }
    Ok(version + 1)
}
