use std::env;
use std::env::VarError;

fn fetch_target() -> Result<String, VarError> {
    env::var("TARGET")
}

fn main() {
    let result = fetch_target();
    match result {
        Ok(s) => println!("Target: {:?}", s),
        Err(e) => println!("Error: {:?}", e),
    }
}
