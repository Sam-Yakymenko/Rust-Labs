use rand::Rng;
use rayon::prelude::*;

const VECTOR_SIZE: usize = 1000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vectors: Vec<Vec<i32>> = Vec::new();

    for _ in 0..VECTOR_SIZE {
        let vector: Vec<i32> = (0..VECTOR_SIZE)
            .map(|_| rng.gen_range(0..100))
            .collect();
        vectors.push(vector);
    }

    let mut results: Vec<i32> = Vec::with_capacity(VECTOR_SIZE);

    vectors.par_iter().map(|vector| {
        vector.iter().sum()
    })
    .collect_into_vec(&mut results);

    println!("Results: {:?}", results);
}
