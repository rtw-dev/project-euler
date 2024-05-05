// https://projecteuler.net/problem=1
// Find the sum of all the multiples of 3 or 5 below 1000.
// Answer: 233168

const BOUNDS: u32 = 1000;

fn main() {
    println!("Simple solution: {}", simple_solution());
    println!("Better solution: {}", better_solution());
}

fn simple_solution() -> u32 {
    /*
    Strategy: directly consider each number in the relevant range
    and add them to a running total if they are a multiple of 3 or 5.
    */
    let mut sum = 0;
    for n in 1..BOUNDS {
        if n % 3 == 0 {
            sum = sum + n;
        } else if n % 5 == 0 {
            sum = sum + n;
        }
    }
    return sum;
}

fn better_solution() -> u32 {
    // Strategy: filter the multiples of 3 and 5 in the relevant range and then sum them.
    return (1..BOUNDS).filter(|&n| n % 3 == 0 || n % 5 == 0).sum();
}
