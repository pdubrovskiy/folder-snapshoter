mod exit;

pub fn run_command(input: i32){
    match input{
        5 => {
            exit::exit();
        }
        _ => {
            println!("Incorrect input. Please repeat your attempt");
        }
    };
}