use std::path::Path;

use crate::snapshot::{self, Snapshot};

use chrono::Local;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::{options::FindOneOptions, Database};

pub async fn create_snapshot(path: &Path, db: &Database) {
    let collection_name = path.to_str().unwrap();
    let collection: Collection<Snapshot> = db.collection(collection_name);

    let version = get_version(&collection).await;
    let date = Local::now().to_string();

    let mut snapshot = snapshot::Snapshot::create(version, date);

    let size = snapshot::fill_and_return_size(path, &mut snapshot.files, &mut snapshot.dirs);
    snapshot.set_size(size);

    db.collection(collection_name)
        .insert_one(snapshot, None)
        .await
        .expect("Insertion Error");
}

pub async fn get_version(collection: &Collection<Snapshot>) -> i32 {
    let find_options = FindOneOptions::builder().sort(doc! {"_id": -1}).build();

    let version = match collection
        .find_one(doc! {}, find_options)
        .await
        .expect("Connection Error")
    {
        Some(document) => document.version + 1,
        None => 1,
    };

    version
}
