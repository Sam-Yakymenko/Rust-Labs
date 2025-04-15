fn is_valid_brackets(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        match char {
            '(' | '[' | '{' => stack.push(char),
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
            _ => {} 
        }
    }

    stack.is_empty()
}

fn main() {
    let test_cases = vec![
        "([]{})[]",
        "([]]",
        "(())",
        "(){}[]",
        "((())",
        "}{",
        "[{()}]",
        "((])",
        "hello(world)",
        "[123]{abc}(xyz)",
        "a[b]c{d}e(f)",
        "(1[2)3]",
        "a(b[c)d}e",
        "раз[два{три(чотири)п'ять}шість]сім",
        "()тест[]123{}",
        "(тест)[123]{}",
        "тест(123)[привіт]{}",
    ];

    for case in test_cases {
        println!("{} : {}", case, is_valid_brackets(case));
    }
}