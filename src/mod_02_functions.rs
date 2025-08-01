pub fn learn_functions() {
    
    println!("Part 2: functions");
    println!("=================");

    // Tuples
    let x = 23;

    // Calling a function
    test_fn("Hello function", x);  // The type of the arguments must correspond

    // Getting a value from a function
    let ret = test_ret(x);  // The type of the arguments must correspond
    println!("Single ret returns {:?}",ret);

    // Getting multiple values from a function
    let (x1,x2,x3) = test_ret_multi(x);  // The type of the arguments must correspond
    println!("Multi ret returns ({x1} {x2} {x3})");

}

// Simple function not returning a value
fn test_fn(s: &str, x:u8) {
    println!("In test_fn function: {} {x}", s );
}

// Simple function returning a value
fn test_ret(x:u8) -> u8 {
    let answer : u8 = x * 2;
    answer // The alast expression witout a semicolon is the return value
}

// Function returning a Tuple
fn test_ret_multi(x:u8) -> (u8,u8,u8) { 
    (x,x*2,x*4)
}
