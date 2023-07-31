use std::env;
use std::path::Path;

use crate::errors::ServiceError;
use crate::snapshot::{self, Snapshot};

use chrono::Local;
use mongodb::Collection;
use mongodb::Database;

pub async fn create_snapshot(path: &Path, db: &Database) -> Result<(), ServiceError> {
    let collection_name = &env::var("COLL_NAME").expect("COLL_NAME must be set");
    let snapshot_path = String::from(path.to_str().unwrap());
    let collection: Collection<Snapshot> = db.collection(&collection_name);

    let version = snapshot::get_version(&collection, &snapshot_path)
        .await
        .expect("Version Error")
        + 1;
    let date = Local::now().to_string();

    let mut snapshot = snapshot::Snapshot::create(version, date, snapshot_path);

    let size = snapshot::fill_and_return_size(path, &mut snapshot.files, &mut snapshot.dirs);
    snapshot.set_size(size);

    match db
        .collection(&env::var("COLL_NAME").expect("COLL_NAME must be set"))
        .insert_one(snapshot, None)
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(ServiceError::FailedToFoundCollection),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::{env, path::PathBuf};

    use mongodb::Collection;

    use crate::commands::snapshot_gallery::get_snapshot;
    use crate::db;
    use crate::snapshot::Snapshot;

    use crate::commands::snapshot_creation::create_snapshot;

    fn set_env_variables() {
        env::set_var("DB_NAME", "snapshots_gallery");
        env::set_var(
            "DB_URI",
            "mongodb+srv://user:user@cluster0.ycjzrmp.mongodb.net/?retryWrites=true&w=majority",
        );
        env::set_var("COLL_NAME", "snapshots");
    }
    #[tokio::test]
    async fn test_create_snapshot() {
        set_env_variables();

        let args: Vec<String> = env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();
        path.push("test_dir");

        fs::create_dir(&path).unwrap();

        let db = db::connect_db().await.unwrap();
        let collection_name = &env::var("COLL_NAME").expect("COLL_NAME must be set");
        let collection: Collection<Snapshot> = db.collection(&collection_name);
        let version = 1;

        create_snapshot(&path, &db).await.unwrap();

        fs::remove_dir_all(&path).unwrap();

        match get_snapshot(&path.to_str().unwrap(), &collection, version)
            .await
            .unwrap()
        {
            Some(_) => true,
            None => false,
        };
    }
}
