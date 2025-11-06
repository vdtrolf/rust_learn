use colored::Colorize;
pub fn print_md(txt_md: [&str; 16]) {
    for l in txt_md {
        let parts = l.split(" ");
        for t in parts {
            if t.starts_with("**") && t.ends_with("**") {
                let u = &t[2..]; // &t.char().count() - 2];
                print!("{} ", u.cyan().bold());
            } else {
                print!("{} ", t);
            }
        }
        println!("");
    }
}

pub fn print_title(title: &str) {
    println!("{}", title.trim().red().bold().underline());
}
