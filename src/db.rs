use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};

use mongodb::error::Error;

pub async fn connect_db() -> Result<Database, Error> {
    let mut client_options =
        ClientOptions::parse(env::var("DB_URI").expect("DB_URI must be set")).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    let db = client.database(&env::var("DB_NAME").expect("DB_NAME must be set"));

    Ok(db)
}

#[cfg(test)]
mod tests {

    use crate::db::connect_db;
    use std::env;

    fn set_env_variables() {
        env::set_var("DB_NAME", "snapshots_gallery");
        env::set_var(
            "DB_URI",
            "mongodb+srv://user:user@cluster0.ycjzrmp.mongodb.net/?retryWrites=true&w=majority",
        );
    }

    #[tokio::test]
    async fn test_connect_db() {
        set_env_variables();
        let result = match connect_db().await {
            Ok(_) => true,
            Err(_) => false,
        };

        assert_eq!(result, true);
    }
}
