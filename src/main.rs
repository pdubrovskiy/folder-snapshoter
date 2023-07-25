use dotenv::dotenv;
use errors::ServiceError;
use std::env;

mod commands;
mod common;
mod db;
mod errors;
mod snapshot;
mod user_interface;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let mut path = common::get_initial_path(&args[0]);

    let db = match db::connect_db().await {
        Ok(db) => db,
        Err(_) => return Err(ServiceError::FailedToCreateDB),
    };

    user_interface::greeting();

    loop {
        user_interface::print_menu();
        commands::run_command(common::get_input(), &mut path, &db).await?;
    }
}
