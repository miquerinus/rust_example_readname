use std::io;
use std::io::prelude::*;

fn main() {

    let mut name = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Error information data ...");

    if let Some('\n') = name.chars().next_back() {

        name.pop();
    }
    
    if let Some('\r') = name.chars().next_back() {

        name.pop();
    }

    println!("Hello, {}\n", name);
    print!("Press <enter ..> for close");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}