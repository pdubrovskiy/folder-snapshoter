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
