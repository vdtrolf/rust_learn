use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 0-Types";
static EXP_TEXT: &str = "
**Types**
> u..
> i..
> f..
> char
> bool
> \"\"
> b\"\" : ^bytestring^ b'' : ^byte char^
> &str : ^(borrowed) sring slice ^
> String::from(\"tata\") : ^String^";

pub fn learn_types(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_types() {
    // Static (referenced to)
    static PI_VALUE: f32 = 3.14162758;

    // Constants (replaced during compil)
    const MAX_VAL: u8 = 128;

    println!("Static: '{PI_VALUE}', Const: '{MAX_VAL}'");

    // Variables
    let mut x: u8 = 11; // mut for mutable
    let y: u8 = 12;
    let z: bool = true; // not used _
    x += y; // AS x is a mutable, this is possible
    let a_char: char = 'f';
    let a_string = "A fixed string"; // immutable string
    let mut a_mut_string = String::from("tes"); // muttable string
    a_mut_string.push('t');

    println!("unsigned: '{x}' bool: '{z}'");
    println!("Char: '{a_char}' Strings: '{a_string}' and a mutable: '{a_mut_string}'");
}
