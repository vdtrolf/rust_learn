use rust_learn::mod_utils::print_md_txt;
use rust_learn::mod_utils::print_title;

static TITLE: &str = " 4-Controls";
static EXP_TEXT: &str = "**simple** : loop {{ if b {{ break; }} }}
**loop**   : let x = loop {{ let b=true; if b {{ break b; }}; }}; \\ x = true
**for**    : let v = vec![]; for i in v {{...}};
**while**  : let mut z=0; while z < 4 {{...}};";

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
        println!("{} - In a outer loop", x);
        let mut y: u8 = 1;
        loop {
            println!("  {}.{} - In a inner loop", x, y);
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
    let mut x = 1;
    let result = loop {
        x += 1;
        if x > 3 {
            break x;
        };
    };
    println!("It ran nearly {result} times!");

    // ITERATING IN VALUES
    let a_vec = vec![2, 4, 8];
    for i in a_vec {
        println!("Next occurence in vector : {i}");
    }

    // ITERATIONS UNTIL A CONDITION IS MET
    let mut z = 0;
    while z < 4 {
        println!("Z is now: {z}");
        z += 1;
    }
    println!("Z is finally: {z}");
}
