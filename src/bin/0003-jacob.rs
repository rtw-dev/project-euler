// https://projecteuler.net/problem=3
// What is the largest prime factor of the number 600,851,475,153?

const NUMBER: i64 = 600851475143;

fn main() {
    /*
    Strategy: Use trial division to find the largest prime factor of the number.
    Note: This method is known to be slow for large numbers!
    */

    let mut factors: Vec<i64> = Vec::new();
    let mut n = NUMBER;
    let mut divisor = 2;

    while n > 1 {
        if n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        } else {
            divisor += 1;
        }
    }

    println!("Simple solution: {}", factors.last().unwrap());
}
