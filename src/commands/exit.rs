use std::process;

#[cfg(not(tarpaulin_include))]
pub fn exit() {
    println!("GoodBye!!!");
    process::exit(1);
}
