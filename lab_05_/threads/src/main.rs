use std::thread;
use std::time::Instant;

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn calculate_fibonacci_range(start: u32, end: u32) {
    let now = Instant::now();

    for i in start..=end {
        let result = fibonacci(i);
        println!("Fibonacci({}) = {}", i, result);
    }

    let duration = now.elapsed();
    println!("Час виконання потоку: {:?}", duration);
}

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let start_range = i * 5;
        let end_range = start_range + 4;

        let handle = thread::spawn(move || {
            calculate_fibonacci_range(start_range, end_range);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}