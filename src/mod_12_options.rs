use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = "12-Options";
static EXP_TEXT: &str = "1. ";

pub fn learn_options(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_options() {
    println!("1. ");
}
