fn clean_phone_number(number: &str) -> String {
    let mut cleaned_number: String = number
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    if cleaned_number.len() == 12 && cleaned_number.starts_with("3") {
        cleaned_number = cleaned_number[2..].to_string();
    }

    cleaned_number
}

fn main() {
    let numbers: Vec<&str> = vec![
        "+3 (050)-995-0253",
        "050-995-0253",
        "3 050 995 0253",
        "050.995.0253",
    ];

    for number in numbers {
        println!("{} -> {}", number, clean_phone_number(number));
    }
}