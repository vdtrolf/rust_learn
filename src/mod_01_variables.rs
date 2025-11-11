use rust_learn::mod_utils::print_md;
use rust_learn::mod_utils::print_title;

static TITLE: &str = " 1-Variables";
static EXP_TEXT: [&str; 16] = [
    "**static** : AAA:f32 = 3.45;",
    "**const**  : BBB:bool = true;",
    "**var**    : let (mut) (_)x:u8 = 11;",
    "**Types**  : u.. i.. f.. char bool \"\"",
    "**Array**  : let an_array:[u8,3]=[1,2,4]; | an_array[1];",
    "**Vector** : let vec = vec![] or let vec = Vec<i16> = vec![-1,2,5,6] | vec[1];",
    "**Tuple**  : let tup = ('a',2,3.24) | tup.1 or let (x1,x2,x3) = tu;",
    "",
    "## test line",
    "",
    "#Another test",
    "",
    "",
    "",
    "",
    "",
];

pub fn learn_variables(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_variables() {
    // Static (referenced to)
    static PI_VALUE: f32 = 3.14162758;

    // Constants (replaced during compil)
    const MAX_VAL: u8 = 128;
    const ARRAY_POS: usize = 1;

    // Variables
    let mut x: u8 = 11; // mut for mutable
    let y: u8 = 12;
    let _z: bool = true; // not used _
    x += y; // AS x is a mutable, this is possible
    let a_char: char = 'f';
    let a_string = "A fixed string"; // immutable string
    let mut a_mut_string = String::from("tes"); // muttable string
    a_mut_string.push('t');

    println!("Hello, world with {a_string}! and a mutable: {a_mut_string}");

    // Arrays
    let tst_array: [u8; 3] = [1, 2, 4];
    let tst_array2 = [0; 3];

    println!("Hello, world {x} / {MAX_VAL} - {tst_array:?}!");
    x -= tst_array[ARRAY_POS];
    println!("And this {} {} {:?}", x, a_char, tst_array2);

    // Vectors
    let vec = vec![1, 2, 3];
    let vec_2: Vec<i16> = vec![-1, 2, 4];
    let elem = vec[1];
    println!("And a vector: {:?} : {elem} : {}", vec, vec_2[0]);

    // Tuples
    let tupl = ("Test", 23, 11.0);
    println!(
        "This is in a tuple value and a static: {}, {PI_VALUE}",
        tupl.1
    );

    let (x1, x2, x3) = tupl;
    println!("Tupl extract {}, {}, {}", x1, x2, x3);

    // This is a code block
    let full_name = {
        let first_name = "John"; // Only visible within the context of the code block
        let last_name = "Doe"; // Same
        format!("{first_name} {last_name}") // Last expression without semicolon = result of the code block
    };

    println!("The code block produced : {full_name}");
}
