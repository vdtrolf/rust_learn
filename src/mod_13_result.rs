use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = "13-Result";
static EXP_TEXT: &str = "1. Result is a standard enum that contains either an OK or an Err
it can be used as return value for functions, and then tested with an if or a match
> fn check_grade(grade : u8) -> Result<(),String> {
>    if grade < 6 { return Ok(()); }
>    return Err(String::from(\"This grade ain't exist\"));
> }
Which can be used in a match
> match check_grade(7) {
>    Ok(_) => println!(\"Student succeded\"), ^( _ means no value )^
>    Err(err_msg) => println!(\"{err_msg}\"),
> }
It is also possible to let Ok() return a value:
> fn check_grade(grade: u8) -> Result<u8, String> ^(return value u8 type is given)^
> ... return Ok(grade * 2); ...
Which can be used in the match (with a test value of 5)
> Ok(return_grade) => println!(\"Student succeeded twice with {}\", return_grade),
The ? operator is a shortcut that propagates the error
> fn division(dividend: f64, divisor: f64) -> Result<String, String> {
>    let result = match divisor {
>        0.0 => Err(String::from(\"Error: Division by zero\")),
>        _ => Ok(dividend / divisor),
>    };
>    let quotient = result?; ^(the ? genarates a break and propagate the error)^
>    Ok(String::from(format!(\"And we have {}\",quotient)))
> }
";

pub fn learn_result(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_result() {
    let grade: u8 = 7;
    match check_grade(grade) {
        Ok(_) => println!("Student succeeded with {}", grade),
        Err(err_msg) => println!("{err_msg}"),
    }

    let grade: u8 = 5;
    match check_grade2(grade) {
        Ok(return_grade) => println!("Student succeeded twice with {}", return_grade),
        Err(err_msg) => println!("{err_msg}"),
    }

    println!("Result: {:?}", division(9.0, 3.0));
    println!("Result: {:?}", division(4.0, 0.0));
}

fn check_grade(grade: u8) -> Result<(), String> {
    if grade < 6 {
        return Ok(());
    }
    return Err(String::from("This grade ain't exist"));
}

fn check_grade2(grade: u8) -> Result<u8, String> {
    if grade < 6 {
        return Ok(grade * 2);
    }
    return Err(String::from("This grade ain't exist"));
}

fn division(dividend: f64, divisor: f64) -> Result<String, String> {
    let result = match divisor {
        0.0 => Err(String::from("Error: Division by zero")),
        _ => Ok(dividend / divisor),
    };
    let quotient = result?;
    Ok(String::from(format!("And we have {}", quotient)))
}
