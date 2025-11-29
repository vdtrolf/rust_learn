use crate::mod_utils::{print_md_txt, print_title};
use std::io;

static TITLE: &str = " 5-Outputs";
static EXP_TEXT: &str = "1. **escape**
> print!(\"1. Start...\");
> print!(\"next part with a '\\t' tab...\");
> print!(\"a double quote \\\", a \\ \");
> print!(\"and the endline \\n \");
2. The carriage return (\\r) moves the cursor back to the line begin
> print!(\"2. First try \\rBut rather this\\n\");
3. **positional**
> println!(\"3. Positional: {} {}\", \"ttt\", 23);
4. **named**
> println!(\"4. Named: {arg1} {arg2}\", arg1 = \"uuu\", arg2 = 34);
5. **input**
> let mut input: String = String::new(); ^(Create a string variable)^
> io::stdin()                            ^(Get the standard input stream)^
>   .read_line(&mut input)               ^(The read_line function reads data until it reaches a '\\n' character)^
>   .expect(\"Unable to read Stdin\");     ^(In case the read operation fails, it panics with the given message)^
> println!(\"5. You entered: {}\", input);";

pub fn learn_outputs(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_outputs() {
    // To print on the same line (btw, this is a simple comment)

    print!("1. Start...");
    print!("next part with a '\t' tab...");
    print!("a double quote \", a \\ ");
    print!("and the endline\n");

    // The carriage return (\r) moves the cursor back to the line begin

    print!("2. First try \rBut rather this\n");

    /*
     * Rust can also use positional arguments {}
     * and named arguments (btw this is a multi-line comment)
     */

    println!("3. Positional: {} {}", "ttt", 23);
    println!("4. Named: {arg1} {arg2}", arg1 = "uuu", arg2 = 34);

    // Input example

    print!("\nInput >>\n");

    let mut input: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

    println!("5. You entered: {}", input);
}
