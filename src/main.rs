mod cond;
mod basics;

use basics::first_part;
use cond::second_part;

fn main() {
    
    println!("In the main function");

    let current_part = 3;
    match current_part {
        1 => first_part(), 
        2 => second_part(),
        _ => println!("No idea"),
    }

}
