use std::io;

fn main() {
    println!("Welcome to the Fibonacci Generator");

    println!("Please input the nth number in the sequence you would like to generate");
    
    let mut nth_fib = String::new();

    io::stdin()
        .read_line(&mut nth_fib)
        .expect("Failed to read line.");

    let nth_fib: u32 = nth_fib.trim().parse().expect("Please input a number!");

    let result = fibonacci(nth_fib);

    println!("The Fibonacci number is: {}", result)
}

fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2) 
    }
}
