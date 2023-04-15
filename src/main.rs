use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <initial_number>", args[0]);
        process::exit(1);
    }

    let initial_number: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid input. Please provide a positive integer.");
            process::exit(1);
        }
    };

    println!("Initial number: {}", initial_number);
    collatz_conjecture(initial_number);
}

fn collatz_conjecture(mut n: u64) {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
        println!("Step {}: {}", steps, n);
    }
}

