// https://projecteuler.net/problem=10
// Find the sum of all the primes below two million.
// Answer: 142913828922

extern crate indicatif;
use indicatif::ProgressBar;

const BOUND: u64 = 2000000;
const PROGRESS_BAR_WIDTH: u64 = 50;

fn main() {
    /*
    Strategy: Will use same method as in solution for problem 7 to generate the primes
    */
    // We need to start with the first prime
    let mut primes: Vec<u64> = vec![2];
    // We will start with the integer larger than our cuurent largest prime
    let mut number: u64 = 3;
    // Loop until we have the primes up to 2 million
    let pb = ProgressBar::new(BOUND as u64);
    loop {
        if number % PROGRESS_BAR_WIDTH == 0 {
            pb.inc(PROGRESS_BAR_WIDTH as u64);
        }
        // If we have the primes up to 2 million, then we can break out of the loop
        if number > BOUND {
            break;
        }
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
            //dbg!(number);
            primes.push(number);
        }
        // Increment the number to test
        number += 1;
    }
    // Output our answer
    pb.finish_with_message("done");
    println!("Simple solution: {}", primes.iter().sum::<u64>());
}
