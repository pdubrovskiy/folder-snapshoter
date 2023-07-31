#[cfg(not(tarpaulin_include))]
pub fn greeting() {
    println!("Welcome to Folder Snapshoter!!!");
}

#[cfg(not(tarpaulin_include))]
pub fn print_menu() {
    println!("|________MENU________|");
    println!("1. navigation");
    println!("2. make snapshot");
    println!("3. snapshot gallery & snapshot comparison");
    println!("4. exit");
}
