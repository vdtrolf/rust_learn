use std::io;

mod mod_01_variables;
mod mod_02_functions;
mod mod_03_conditions;
mod mod_04_controls;
mod mod_05_outputs;
mod mod_06_ownership;
mod mod_07_borrowing;
mod mod_08_dereferencing;

use mod_01_variables::learn_variables;
use mod_02_functions::learn_functions;
use mod_03_conditions::learn_conditions;
use mod_04_controls::learn_controls;
use mod_05_outputs::learn_outputs;
use mod_06_ownership::learn_ownership;
use mod_07_borrowing::learn_borrowing;
use mod_08_dereferencing::learn_dereferencing;


// https://codezup.com/rust-memory-management-guide/

fn main() {
    
    println!("LEARN RUST");
    println!("Which module (1..8) ?");

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
        "8" => learn_dereferencing(),
        _ => println!("No idea"),
    }

}
