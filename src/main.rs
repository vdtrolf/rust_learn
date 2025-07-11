mod variables;
mod conditions;
mod functions;
mod controls;

use variables::learn_variables;
use functions::learn_functions;
use conditions::learn_conditions;
use controls::learn_controls;

fn main() {
    
    println!("LEARN RUST");
    println!("");

    let current_part = 4;
    match current_part {
        1 => learn_variables(), 
        2 => learn_functions(),
        3 => learn_conditions(),
        4 => learn_controls(),
        _ => println!("No idea"),
    }

}
