use std::env;
use std::str::FromStr;

use rust_gcd as gcd;

fn main() {
    let numbers = &mut Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let d = gcd::compute(numbers);
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
