mod user_interface;
mod commands;
fn main() {
    user_interface::greeting();

    loop {
        user_interface::print_menu();
        commands::run_command(user_interface::get_input());
    }
}





