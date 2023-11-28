use std::env;

fn sieve_of_eratosthenes(max_prime: usize) -> Vec<usize> {
    let mut primes = vec![true; max_prime + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=((max_prime as f64).sqrt() as usize) {
        if primes[i] {
            let mut j = i * i;
            while j <= max_prime {
                primes[j] = false;
                j += i;
            }
        }
    }

    primes.iter().enumerate().filter_map(|(i, &is_prime)| {
        if is_prime { Some(i) } else { None }
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: prime_calculator <max_prime> <num_iterations>");
        return;
    }

    let max_prime = args[1].parse::<usize>().expect("Error parsing max_prime");
    let num_iterations = args[2].parse::<usize>().expect("Error parsing num_iterations");

    for _ in 0..num_iterations {
        let primes = sieve_of_eratosthenes(max_prime);
        println!("Found {} primes up to {}", primes.len(), max_prime);
    }
}


