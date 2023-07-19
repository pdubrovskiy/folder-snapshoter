use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};

const DB_NAME: &str = "snapshots_gallery";
const DB_URI: &str =
    "mongodb+srv://user:user@cluster0.ycjzrmp.mongodb.net/?retryWrites=true&w=majority";

pub async fn connect_db() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(DB_URI).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    let db = client.database(DB_NAME);

    Ok(db)
}
