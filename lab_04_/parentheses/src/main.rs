use std::env;

fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.is_empty() || stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.is_empty() || stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.is_empty() || stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {} // Ignore other characters
        }
    }

    stack.is_empty()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: parentheses <string>");
        return;
    }

    let input_string = &args[1];

    if is_balanced(input_string) {
        println!("Brackets are balanced");
    } else {
        println!("Brackets are not balanced");
    }
}
