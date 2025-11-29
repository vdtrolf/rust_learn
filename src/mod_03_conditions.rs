use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 3-Conditions";
static EXP_TEXT: &str = "**1. if**
> if exp {...} else if exp {...} else {...}; ^( 'exp' like 'x>1' )^
**2. test on a boolean**
> let b:bool=true;
> if b { println!(\"I am in\")};
**3. match on values**
> let x: u8 = 3;
> match x {
>   1 => println!(\"It's a one\"),
>   2 | 3 => println!(\"It's not a one\"),
>   _ => (), ^( _ is the default, () means do nothing )^
> }; ^( watch out for the commas )^
**4. match on range**
> let y = 2.5;
> match y {
>   0.0 .. 2.0 => println!(\"Between 0 and 2 (exclusive)\"), ^( excluding 0.0 and 2.0 )^
>   2.0 ..= 3.0 => println!(\"Between 0 and 2 (including 3.0)\"), ^( including 3.0 )^
>   _ => (),
> };
**5.match to return a value**
> let b = match x {
>   1 | 3 => \"uneven\",
>   2 | 4 => \"even\",
> }; ^( if x=2 then b gets even)^
> println!(\"It is an {}\",b);";

pub fn learn_conditions(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_conditions() {
    // IF
    let x = 1;
    let a_str = {
        let a_val = 2; // a_val is only visible in the code block
        if x < a_val {
            "aa"
        } else if x == a_val {
            "bb"
        } else {
            "no winner"
        }
    };
    println!("1. And the winner of the 'if' test is: {}", a_str);

    // IF on a boolean
    let b: bool = true;
    if b {
        println!("2. I am in")
    };

    // MATCH
    let x: u8 = 3;
    match x {
        1 => println!("3. It's a one"),
        2 | 3 => println!("3. It's not a one"),
        _ => (), // _ is the default, () means do nothing
    }; // watch out for the commas

    // MATCH ON RANGE
    let y = 2.5;
    match y {
        0.0..2.0 => println!("4. Between 0 and 2 (exclusive)"), // excluding 0.0 and 2.0
        2.0..=3.0 => println!("4. Between 2 and 3 (including 3.0)"), // including 3.0
        _ => (),
    };

    // MATCH AS VALUE
    let b = match x {
        1 | 3 => "uneven",
        2 | 4 => "even",
        _ => "unknown",
    }; // if x=2 then b gets even
    println!("5. It is an {}", b);
}
