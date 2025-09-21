use std::io;

mod mod_01_variables;
mod mod_02_functions;
mod mod_03_conditions;
mod mod_04_controls;
mod mod_05_outputs;
mod mod_06_ownership;
mod mod_07_borrowing;
mod mod_08_dereferencing;
mod mod_09_struct;
mod mod_10_implementation;

use mod_01_variables::learn_variables;
use mod_01_variables::test_variables;
use mod_02_functions::learn_functions;
use mod_02_functions::test_functions;
use mod_03_conditions::learn_conditions;
use mod_03_conditions::test_conditions;
use mod_04_controls::learn_controls;
use mod_04_controls::test_controls;
use mod_05_outputs::learn_outputs;
use mod_05_outputs::test_outputs;
use mod_06_ownership::learn_ownership;
use mod_06_ownership::test_ownership;
use mod_07_borrowing::learn_borrowing;
use mod_07_borrowing::test_borrowing;
use mod_08_dereferencing::learn_dereferencing;
use mod_08_dereferencing::test_dereferencing;
use mod_09_struct::learn_struct;
use mod_09_struct::test_struct;
use mod_10_implementation::learn_implementation;
use mod_10_implementation::test_implementation;

// https://codezup.com/rust-memory-management-guide/

fn main() {
    println!("LEARN RUST");
    println!("Which module (1 to 9) ?");

    let mut input: String = String::new();
    let mut last: String = String::new();

    while input.trim() != "0" {
        input = String::new();
        io::stdin() // Get the standard input stream
            .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

        match input.trim() {
            "0" => println!("exit"),
            "1" => learn_variables(),
            "2" => learn_functions(),
            "3" => learn_conditions(),
            "4" => learn_controls(),
            "5" => learn_outputs(),
            "6" => learn_ownership(),
            "7" => learn_borrowing(),
            "8" => learn_dereferencing(),
            "9" => learn_struct(),
            "10" => learn_implementation(),
            "t" => match last.trim() {
                "1" => test_variables(),
                "2" => test_functions(),
                "3" => test_conditions(),
                "4" => test_controls(),
                "5" => test_outputs(),
                "6" => test_ownership(),
                "7" => test_borrowing(),
                "8" => test_dereferencing(),
                "9" => test_struct(),
                "10" => test_implementation(),
                _ => println!("nope"),
            },

            _ => println!("No idea"),
        }
        last = input.clone();
    }
}
