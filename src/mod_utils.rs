use colored::Colorize;
pub fn print_md(txt_md: [&str; 16]) {
    for l in txt_md {
        if l.starts_with("###") {
            println!("{}", l[3..].trim().on_red().bold());
        } else if l.starts_with("##") {
            println!("{}", l[2..].trim().yellow().bold());
        } else if l.starts_with("#") {
            println!("{}", l[1..].trim().green().bold());
        } else {
            let parts = l.split(" ");
            let mut inb: bool = false;
            let mut buf: String = ("").to_string();
            for t in parts {
                if t.starts_with("**") && t.ends_with("**") {
                    let mut s = t[2..].to_string();
                    s.pop();
                    s.pop();
                    print!("{} ", s.cyan().bold());
                } else if t.starts_with("**") {
                    inb = true;
                    buf = (&t[2..]).to_string();
                } else if t.ends_with("**") && inb {
                    let mut newbuf = buf.to_string() + " " + t;
                    newbuf.pop();
                    newbuf.pop();
                    print!("{} ", newbuf.cyan().bold());
                    buf = ("").to_string();
                    inb = false;
                } else if inb {
                    let newbuf = buf.to_string() + " " + t;
                    buf = newbuf;
                } else {
                    print!("{} ", t);
                }
            }
        }
        println!("");
    }
}

pub fn print_title(title: &str) {
    println!("{}", title.trim().red().bold().underline());
}
