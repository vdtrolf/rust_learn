// Function returning a Tuple

pub fn learn_conditions() { 
    println!("In part 3: conditions");
    println!("=====================");

    // IF

    let x = 1;
    let a_str = {
        let a_val = 2; // a_val is only visible in the code block
        if x < a_val  {
            "aa"
        } else if x == a_val {
            "bb"
        } else {
            "no winner"
        }
    };
    println!("And the winner of the 'if' test is: {}", a_str);

    // IF on a boolean

    let b = true;
    if !b { 
        println!("This will never come on the screen")
    }

    // MATCH

    let y = 5.0;

    let a_char = match y {
        0.0 .. 2.0 => 'b', // from 0.0 to 2.0 (excl) 
        2.0 => 'e', // exactly 2.0
        2.0 .. 3.0 => 'a', // from 2.0 to 3.0 (excl)
        3.0 ..= 5.0 => 'f', // from 3.0 to 5.0 (incl) - was '...' before
        _ => 'o',
    };
    println!("And the result of the first 'match' test is: {}", a_char);

    let z = 12;
    let b_str = match z {
        2 | 4 | 6 | 8 => "even",
        1 | 3 | 5 | 7 | 9 => "uneven",
        0 => "zero",
        _ => "unknown",
    };
    println!("And the result of the second 'match' test is: {}", b_str);

}
