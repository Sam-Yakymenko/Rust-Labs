fn main() {
    let size: i32 = 10;  

    print!("    ");  
    for i in 1..=size {
        print!("{:4}", i); 
    }
    println!(); 

    for i in 1..=size {
        print!("{:3} |", i); 
        for j in 1..=size {
            print!("{:4}", i * j);  
        }
        println!(); 
    }
}