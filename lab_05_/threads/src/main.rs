use std::thread;
use std::time::Instant;
use rand::Rng;
use std::collections::HashMap;

// Function to calculate Fibonacci numbers in a range
fn fibonacci_range(start: u64, end: u64) {
    let start_time = Instant::now();
    for i in start..=end {
        let fib = fibonacci(i);
        println!("Fibonacci of {} is {}", i, fib);
    }
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// Function to generate a random array
fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-200..100)).collect()
}

// Function to find the most frequent element in an array (sequential)
fn find_most_frequent_sequential(arr: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &num in arr {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut most_frequent = 0;
    let mut max_count = 0;
    for (&num, &count) in &counts {
        if count > max_count {
            most_frequent = num;
            max_count = count;
        }
    }
    most_frequent
}

// Function to find the most frequent element in an array (parallel)
fn find_most_frequent_parallel(arr: &Vec<i32>, num_threads: usize) -> i32 {
    let chunk_size = arr.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            arr.len()
        } else {
            (i + 1) * chunk_size
        };

        let chunk = arr[start..end].to_vec();

        let handle = thread::spawn(move || {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for &num in &chunk {
                *counts.entry(num).or_insert(0) += 1;
            }
            counts
        });
        handles.push(handle);
    }

    let mut combined_counts: HashMap<i32, usize> = HashMap::new();
    for handle in handles {
        let counts = handle.join().unwrap();
        for (num, count) in counts {
            *combined_counts.entry(num).or_insert(0) += count;
        }
    }

    let mut most_frequent = 0;
    let mut max_count = 0;
    for (&num, &count) in &combined_counts {
        if count > max_count {
            most_frequent = num;
            max_count = count;
        }
    }
    most_frequent
}


fn main() {
    // Fibonacci calculation in threads
    let handle1 = thread::spawn(|| {
        fibonacci_range(1, 10);
    });

    let handle2 = thread::spawn(|| {
        fibonacci_range(11, 20);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Array processing
    let array_size = 100_000;
    let arr = generate_random_array(array_size);

    // Sequential processing
    let start_time = Instant::now();
    let most_frequent_sequential = find_most_frequent_sequential(&arr);
    let duration_sequential = start_time.elapsed();

    println!("Most frequent element (sequential): {}", most_frequent_sequential);
    println!("Time taken (sequential): {:?}", duration_sequential);

    // Parallel processing
    let num_threads = 4;
    let start_time = Instant::now();
    let most_frequent_parallel = find_most_frequent_parallel(&arr, num_threads);
    let duration_parallel = start_time.elapsed();

    println!("Most frequent element (parallel): {}", most_frequent_parallel);
    println!("Time taken (parallel with {} threads): {:?}", num_threads, duration_parallel);
}
