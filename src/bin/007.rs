// https://projecteuler.net/problem=7
// What is the 10,001st prime number?
// Answer: 104743

const BOUND: usize = 10001;

fn main() {
    /*
    Strategy: We will just generate all the primes, returning the 10,001st that we find.
    */

    // We need to start with the first primes
    let mut primes: Vec<u64> = vec![2];
    // We will start with the integer larger than our cuurent largest prime
    let mut number = 3;
    // We will keep generating primes until we have the 10,001st
    while primes.len() < BOUND {
        // Check if the current number is prime by trial division
        let mut is_prime = true;
        for prime in &primes {
            // If any known prime divides the current number, then it is not prime
            if number % prime == 0 {
                is_prime = false;
                break; // No need to check if any more known primes divide into the number
            }
        }
        // If the number is prime, then add it to our list of primes
        if is_prime {
            primes.push(number);
        }
        // Increment the number to test
        number += 1;
    }
    // Output our answer
    println!("Simple solution: {}", primes.pop().unwrap());
}
