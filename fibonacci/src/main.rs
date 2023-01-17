use std::io;

fn main() {
    println!("Fibonacci Sequence");
    println!("Please enter a number");

    let mut n = String::new();

    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("The term in Fibonacci Sequence is {}",fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}