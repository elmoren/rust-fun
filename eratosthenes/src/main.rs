// Seive of Eratosthenes
// Simple prime number generation algorithm

// ... My first Rust program

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("usage:: Finds all primes up to n: missing arg <n>");
	return;
    }
    
    let input = args[1].parse::<i32>();
    
    let n = match input {
	Ok(n) => {n},
	Err(e) => {
	    println!("Error: {:?}", e);
	    return;
	}
    };
    
    println!("Finding all primes up to {:?}", n);
    
    let mut primes: Vec<i32> = (0..n).collect();

    sieve (&mut primes);

    for p in primes {
	if p > 1 {
	    println!("{}", p);
	}
    }

}

fn sieve (primes: &mut Vec<i32>) {
    for i in 2..primes.len() {
	if primes[i] > 0 {
	    let mut j = i << 1;
	    while j < primes.len() {
		primes[j] = 0;
		j += i;
	    }
	}
    }
}
