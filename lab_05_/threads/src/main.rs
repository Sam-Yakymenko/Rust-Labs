use std::thread;
use std::time::Instant;
use rand::Rng;
use std::collections::HashMap;
use std::io;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn run_fibonacci_task() {
    println!("Обчислення Фібоначчі в потоках:");
    let num_threads_fibonacci = 4;

    let mut fibonacci_handles = vec![];

    for i in 0..num_threads_fibonacci {
        let start_range = i * 5;
        let end_range = (i + 1) * 5;

        let handle = thread::spawn(move || {
            let start_time = Instant::now();
            println!("Потік {} розпочав обчислення", i);

            for n in start_range..end_range {
                println!("Fib({}) = {}", n, fib(n as u32));
            }

            let duration = start_time.elapsed();
            println!("Потік {} завершив за {:?}", i, duration);
        });

        fibonacci_handles.push(handle);
    }

    for handle in fibonacci_handles {
        handle.join().unwrap();
    }
}

fn run_array_processing_task() {
    println!("\nОбробка масиву:");
    let num_threads_array = 4;
    let array_size = 1000;
    let mut arr: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..array_size {
        arr.push(rng.gen_range(-200..=100));
    }

    println!("Послідовна обробка почалася");
    let start_time_seq = Instant::now();
    let mut most_frequent_seq = 0;
    let mut max_count_seq = 0;
    for &num in &arr {
        let count = arr.iter().filter(|&n| *n == num).count();
        if count > max_count_seq {
            max_count_seq = count;
            most_frequent_seq = num;
        }
    }
    
    let duration_seq = start_time_seq.elapsed();
    println!("Послідовна обробка: найбільш частий елемент = {}, час = {:?}", most_frequent_seq, duration_seq);
    println!("Послідовна обробка завершилася");

    println!("Паралельна обробка почалася");
    let start_time_par = Instant::now();
    let mut array_handles = vec![];
    let chunk_size = array_size / num_threads_array;

    for i in 0..num_threads_array {
        let start = i * chunk_size;
        let end = if i == num_threads_array - 1 {
            array_size
        } else {
            (i + 1) * chunk_size
        };

        let chunk = arr[start..end].to_vec();

        let handle = thread::spawn(move || {
            let mut counts = HashMap::new();
            for &num in &chunk {
                *counts.entry(num).or_insert(0) += 1;
            }
            let result = counts.into_iter().max_by_key(|(_, count)| *count);
            result
        });
        array_handles.push(handle);
    }

    let mut max_count_par = 0;
    let mut most_frequent_par = 0;

    for handle in array_handles {
        if let Some((thread_most_frequent, thread_max_count)) = handle.join().unwrap() {
            if thread_max_count > max_count_par {
                max_count_par = thread_max_count;
                most_frequent_par = thread_most_frequent;
            }
        }
    }

    let duration_par = start_time_par.elapsed();
    println!("Паралельна обробка: найбільш частий елемент = {}, час = {:?}", most_frequent_par, duration_par);
    println!("Паралельна обробка завершилася");
}

fn main() {
    loop {
        println!("Оберіть завдання:");
        println!("1. Обчислення чисел Фібоначчі");
        println!("2. Обробка масиву");
        println!("0. Вихід");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Помилка читання з stdin");

        let choice = input.trim();

        match choice {
            "1" => run_fibonacci_task(),
            "2" => run_array_processing_task(),
            "0" => break,
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}