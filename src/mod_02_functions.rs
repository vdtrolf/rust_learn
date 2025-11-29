use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 2-Functions";
static EXP_TEXT: &str = "**1. simple**
> test_fn(\"tata\",1);
> fn test_fn(s: &str, x:u8) {
>   println!(\"1. In test_fn function: {} {x}\", s);
> }
**2. param**
> let ret = test_ret(1);
> fn test_ret(x: u8) -> u8 {
>   let answer: u8 = x * 2;
>   answer ^(The last expression witout a semicolon is the return value)^
> }
**3. multi return**
> let (x1,x2) = test_ret_multi(1);
> fn test_ret_multi(x: u8) -> (u8, u8) {
>   (x * 2, x * 4) ^(returns a tuple)^
> }";

pub fn learn_functions(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_functions() {
    let x: u8 = 1;

    // Calling a function
    test_fn("tata", x); // The type of the arguments must correspond

    // Getting a value from a function
    let ret = test_ret(x); // The type of the arguments must correspond
    println!("2. Single ret returns {:?}", ret);

    // Getting multiple values from a function
    let (x1, x2) = test_ret_multi(x); // The type of the arguments must correspond
    println!("3. Multi ret returns ({x1} {x2})");
}

// Simple function not returning a value
fn test_fn(s: &str, x: u8) {
    println!("1. In test_fn function: {} {x}", s);
}

// Simple function returning a value
fn test_ret(x: u8) -> u8 {
    let answer: u8 = x * 2;
    answer // The alast expression witout a semicolon is the return value
}

// Function returning a Tuple
fn test_ret_multi(x: u8) -> (u8, u8) {
    (x * 2, x * 4)
}
