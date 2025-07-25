use std::io;

mod variables;
mod conditions;
mod functions;
mod controls;
mod outputs;
mod ownership;
mod borrowing;

use variables::learn_variables;
use functions::learn_functions;
use conditions::learn_conditions;
use controls::learn_controls;
use outputs::learn_outputs;
use ownership::learn_ownership;
use borrowing::learn_borrowing;

fn main() {
    
    println!("LEARN RUST");
    println!("Which module (1..7) ?");

    let mut input: String = String::new();
    
    io::stdin() // Get the standard input stream
        .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
    
    match input.trim() {
        "1" => learn_variables(), 
        "2" => learn_functions(),
        "3" => learn_conditions(),
        "4" => learn_controls(),
        "5" => learn_outputs(),
        "6" => learn_ownership(),
        "7" => learn_borrowing(),
        _ => println!("No idea"),
    }

}
