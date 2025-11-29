use colored::Colorize;
use inquire::Text;
use rust_learn::mod_utils::print_md_txt;
use std::fs;

use rust_learn::mod_00_types::{learn_types, test_types};
use rust_learn::mod_01_variables::{learn_variables, test_variables};
use rust_learn::mod_02_functions::{learn_functions, test_functions};
use rust_learn::mod_03_conditions::{learn_conditions, test_conditions};
use rust_learn::mod_04_controls::{learn_controls, test_controls};
use rust_learn::mod_05_outputs::{learn_outputs, test_outputs};
use rust_learn::mod_06_ownership::{learn_ownership, test_ownership};
use rust_learn::mod_07_borrowing::{learn_borrowing, test_borrowing};
use rust_learn::mod_08_dereferencing::{learn_dereferencing, test_dereferencing};
use rust_learn::mod_09_struct::{learn_struct, test_struct};
use rust_learn::mod_10_implementation::{learn_implementation, test_implementation};
use rust_learn::mod_11_enums::{learn_enums, test_enums};
use rust_learn::mod_12_options::{learn_options, test_options};
use rust_learn::mod_13_result::{learn_result, test_result};
use rust_learn::mod_14_hashes::{learn_hashes, test_hashes};

fn main() {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print_index();

    let mut last: String = String::new();

    loop {
        println!("");
        let name = Text::new("0 to 14 ?").prompt();

        match name {
            Ok(name) => {
                if process_val(name.as_str(), last.as_str()) {
                    break;
                };
                last = name.clone();
            }
            Err(_) => break,
        }
    }
}

fn process_val(input: &str, last: &str) -> bool {
    let mut do_break: bool = false;

    if input.trim() != "t" && input.trim() != "m" && input.trim() != "T" && input.trim() != "M" {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    match input.trim() {
        "a" | "A" => print_index(),
        "m" | "M" => read_memory_md(),
        "q" | "Q" => do_break = true,
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
        "11" => learn_enums(true),
        "12" => learn_options(true),
        "13" => learn_result(true),
        "14" => learn_hashes(true),
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
            "11" => test_enums(),
            "12" => test_options(),
            "13" => test_result(),
            "14" => test_hashes(),
            _ => (),
        },
        _ => println!("No idea"),
    }
    return do_break;
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
    learn_enums(false);
    learn_options(false);
    learn_result(false);
    learn_hashes(false);
    println!("");
    println!("Which module (1 to 14) ?");
}

fn read_memory_md() {
    let file_path = "./src/memory.md";
    let contents = fs::read_to_string(file_path).expect("Can't read the file memory.md");
    print_md_txt(contents.as_str());
}
