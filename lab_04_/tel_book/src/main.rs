use std::io;
use std::io::BufRead;

fn clean_phone_number(number: &str) -> String {
    let mut cleaned_number = number.replace(|c: char| !c.is_numeric(), "");
    if cleaned_number.starts_with("3") {
        cleaned_number = cleaned_number[1..].to_string();
    }
    cleaned_number
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cleaned_number = clean_phone_number(&line);
        println!("{}", cleaned_number);
    }
}
