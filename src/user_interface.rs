use std::io;

pub fn greeting() {
    println!("Welcome to Folder Snapshoter!!!");
}

pub fn print_menu() {
    println!("|________MENU________|");
    println!("1. navigation");
    println!("2. make snapshot");
    println!("3. snapshot gallery");
    println!("4. compare snapshots");
    println!("5. exit");
}

pub fn get_input() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Input Error");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Incorrect input. Please repeat your attempt");
                print_menu();
                continue;
            }
        };
    }
}