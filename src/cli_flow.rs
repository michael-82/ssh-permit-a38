use colored::Colorize;
use std::io;
use std::io::Write;
use std::process::exit;

pub fn errorln(msg: &str) {
    println!("{} {}", "Error:".red(), msg);
    exit(1);
}

pub fn prompt(msg: &str, colorful: bool) {
    if colorful {
        print!("{} ", msg.yellow());
    } else {
        print!("{} ", msg);
    }
    io::stdout().flush().expect("Unable to flush");
}

pub fn promptln(msg: &str) {
    println!("{}", msg);
}

pub fn prompt_yes_no(msg: &str, colorful: bool) -> String {
    let mut yes_no;

    loop {
        yes_no = String::from("");

        prompt(&mut format!("{}", msg), colorful);
        io::stdin()
            .read_line(&mut yes_no)
            .ok()
            .expect("Couldn't read line (y/n)");

        yes_no = yes_no.trim_end().trim_start().to_owned();
        if yes_no == "n" || yes_no == "y" {
            break;
        }
    }

    yes_no
}

pub fn read_line<'a>(msg: &str, default: &'a String) -> String {
    prompt(msg, false);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read_line");

    input = input.trim_end().trim_start().to_string();
    if !input.is_empty() {
        return input;
    }

    default.to_owned()
}

pub fn okln(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn warningln(msg: &str) {
    println!("{} {}", "Warning:".magenta(), msg);
}

pub fn infoln(msg: &str) {
    println!("{}", msg);
}
