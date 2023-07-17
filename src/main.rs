use std::env;

mod user_interface;
mod commands;
mod common;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = common::get_initial_path(&args[0]);
    
    user_interface::greeting();

    loop {
        user_interface::print_menu();
        commands::run_command(common::get_input(), &mut path);
    }
}