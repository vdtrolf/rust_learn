use colored::Colorize;
pub fn print_md(txt_md: [&str; 16]) {
    for l in txt_md {
        println!("{}", l);
    }
}

pub fn print_title(title: &str) {
    println!("{}", title.trim().red().bold().underline());
}
