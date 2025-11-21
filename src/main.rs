use colored::Colorize;
use rust_learn::mod_utils::print_md_txt;
use std::fs;
use std::io;

mod mod_00_types;
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

use mod_00_types::learn_types;
use mod_00_types::test_types;
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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print_index();

    let mut last: String = String::new();

    loop {
        let mut input: String = String::new();
        io::stdin() // Get the standard input stream
            .read_line(&mut input) // The rea read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
        if input.trim() != "t" && input.trim() != "m" && input.trim() != "T" && input.trim() != "M"
        {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        match input.trim() {
            "a" | "A" => print_index(),
            "m" | "M" => read_memory_md(),
            "q" | "Q" => break,
            "0" => learn_types(true),
            "1" => learn_variables(true),
            "2" => learn_functions(true),
            "3" => learn_conditions(true),
            "4" => learn_controls(true),
            "5" => learn_outputs(true),
            "6" => learn_ownership(true),
            "7" => learn_borrowing(true),
            "8" => learn_dereferencing(true),
            "9" => learn_struct(true),
            "10" => learn_implementation(true),
            "t" | "T" => match last.trim() {
                "0" => test_types(),
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
        println!("");
        println!("Which module ?");
        last = input.clone();
    }
}

fn print_index() {
    println!("{}", "LEARN RUST".red().bold());
    println!("");
    learn_types(false);
    learn_variables(false);
    learn_functions(false);
    learn_conditions(false);
    learn_controls(false);
    learn_outputs(false);
    learn_ownership(false);
    learn_borrowing(false);
    learn_dereferencing(false);
    learn_struct(false);
    learn_implementation(false);
    println!("");
    println!("Which module (1 to 10) ?");
}

fn read_memory_md() {
    let file_path = "./src/memory.md";
    let contents = fs::read_to_string(file_path).expect("Can't read the file memory.md");
    print_md_txt(contents.as_str());
}
