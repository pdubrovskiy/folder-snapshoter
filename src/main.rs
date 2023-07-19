use std::env;

mod commands;
mod common;
mod db;
mod snapshot;
mod user_interface;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = common::get_initial_path(&args[0]);

    let db = db::connect_db().await?;

    user_interface::greeting();

    loop {
        user_interface::print_menu();
        commands::run_command(common::get_input(), &mut path, &db).await;
    }
}
