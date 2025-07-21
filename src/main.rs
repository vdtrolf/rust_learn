mod variables;
mod conditions;
mod functions;
mod controls;
mod outputs;

use variables::learn_variables;
use functions::learn_functions;
use conditions::learn_conditions;
use controls::learn_controls;
use outputs::learn_outputs;

fn main() {
    
    println!("LEARN RUST");
    println!("");

    let current_part = 5;
    match current_part {
        1 => learn_variables(), 
        2 => learn_functions(),
        3 => learn_conditions(),
        4 => learn_controls(),
        5 => learn_outputs(),
        _ => println!("No idea"),
    }

}
