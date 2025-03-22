fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

fn print_matrix(matrix: [[i32; 3]; 3]) {
    for row in matrix.iter() {
        for &element in row {
            print!("{} ", element);
        }
        println!();
    }
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303]
    ];
    println!("Original matrix:");
    print_matrix(matrix);
    
    println!("\nTransposed matrix:");
    print_matrix(transpose(matrix));
}