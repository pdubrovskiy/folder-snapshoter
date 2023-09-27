use std::path::PathBuf;

use mongodb::Database;

use crate::errors::ServiceError;

mod exit;
mod navigation;
mod snapshot_creation;
mod snapshot_gallery;

pub async fn run_command(
    input: i32,
    path: &mut PathBuf,
    db: &Database,
) -> Result<(), ServiceError> {
    match input {
        1 => {
            navigation::nav_menu(path);
        }
        2 => {
            snapshot_creation::create_snapshot(path, db).await?;
        }
        3 => {
            snapshot_gallery::snapshot_gallery_menu(path, db).await?;
        }
        4 => {
            exit::exit();
        }
        _ => {
            println!("Incorrect input. Please repeat your attempt");
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::{commands::run_command, db};

    fn set_env_variables() {
        env::set_var("DB_NAME", "snapshots_gallery");
        env::set_var(
            "DB_URI",
            "mongodb+srv://user:user@cluster0.ycjzrmp.mongodb.net/?retryWrites=true&w=majority",
        );
    }

    #[tokio::test]
    async fn test_run_command_incorrect_input() {
        set_env_variables();

        let db = db::connect_db().await.unwrap();
        let mut path = PathBuf::new();
        let result = run_command(5, &mut path, &db).await.unwrap();

        assert_eq!(result, ())
    }
}
