fn main(){
    let n:u32 = 20;
    println!("fib({}) = {}", n, fib(n));
  }
  
  fn fib(n:u32) -> u32{
    if n <= 1{
      n
    }
    else{
      fib(n-1) + fib(n-2)
    }
  }