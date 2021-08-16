use std::io;

fn main() {
    let mut n = String::new();

    println!("Please enter the which Fibonacci number you want to calculate");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    
    let n: f32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Invalid input"); 0.0},
    };

    let golden_ratio:f32 = 1.61803398875;

    let fib = ((golden_ratio.powf(n) - ((1 as f32)-golden_ratio).powf(n))/(5 as f32).sqrt()) as u32;

    println!("Your fibbonacci number sir, {}", fib);
}
