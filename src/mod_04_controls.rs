use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 4-Controls";
static EXP_TEXT: &str = "**1. simple**
> loop { if b { break; } }
**2. loop as value**
> let x = loop {
>   let b=true;
>   if b { break b };
> };
> println!(\"2. x is now {}\", x); ^( x = true )^
**3. for**
> let a_vec = vec![2, 4, 8];
> for i in a_vec {
>   println!(\"3. next occurence in vector : {i}\");
> }
**4. while**
> let mut z = 0;
> while z < 4 {
>     println!(\"4. z is now: {z}\");
>     z += 1;
> }
> println!(\"4. z is finally: {z}\");";

pub fn learn_controls(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_controls() {
    
    // SIMPLE LOOP
    
    let mut x: u8 = 1;
    loop {
        println!("1. {} - In a outer loop", x);
        let mut y: u8 = 1;
        loop {
            println!("1. {}.{} - In a inner loop", x, y);
            y += 1;
            if y > 2 {
                break; // breaks the inner loop
            }
        }
        x += 1;
        if x > 2 {
            break; // breaks the outer loop
        }
    }

    // LOOP AS AN EXPRESSION
    let x = loop {
        let b = true;
        if b {
            break b;
        };
    };
    println!("2. x is now {}", x); //  x = true

    // ITERATING IN VALUES
    let a_vec = vec![2, 4, 8];
    for i in a_vec {
        println!("3. Next occurence in vector : {i}");
    }

    // ITERATIONS UNTIL A CONDITION IS MET
    let mut z = 0;
    while z < 4 {
        println!("4. z is now: {z}");
        z += 1;
    }
    println!("4. z is finally: {z}");
}
