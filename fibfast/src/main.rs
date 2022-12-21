extern crate num_bigint;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::io;

fn fibonacci(n: u64, cache: &mut HashMap<u64, BigUint>) -> BigUint {
    if n == 0 {
        return BigUint::from(0u64);
    } else if n == 1 {
        return BigUint::from(1u64);
    } else {
        let result = cache.get(&n);
        match result {
            Some(x) => x.clone(),
            None => {
                let res = &fibonacci(n-1, cache) + &fibonacci(n-2, cache);
                cache.insert(n, res.clone());
                res
            }
        }
    }
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: u64 = input.trim().parse().expect("Please type a number!");

    let mut cache: HashMap<u64, BigUint> = HashMap::new();
    println!("The {}th number in the Fibonacci series is {}", n, fibonacci(n, &mut cache));
}

