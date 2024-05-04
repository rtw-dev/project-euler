// https://projecteuler.net/problem=5
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

const BOUND: i64 = 20;

fn main() {
    /*
    Strategy: Use known mathematical facts about Least Common Multiples (LCFs) and Greatest Common Divisors (GCDs):
    - LCM(a, b) = (a * b) / GCD(a, b))
    - GCD(a, b) = GCD(b, a mod b) - the Euclidean Algorithm
    - LCM(a, b, c) = LCM(a, LCM(b,c)) - so we can use a recursive approach
    */

    // Recursive approach is easy to understand
    let numbers: Vec<i64> = (1..=BOUND).collect();
    let solution_by_recursion: i64 = lcm_of_numbers(numbers);
    println!("Simple solution: {}", solution_by_recursion);

    // Folding solution requires a bit more thought
    let solution_by_folding: i64 = (1..=BOUND).fold(1, |acc, n| (acc * n) / gcd(acc, n));
    println!("Better solution: {}", solution_by_folding);
}

fn lcm_of_numbers(numbers: Vec<i64>) -> i64 {
    // If only two numbers in the list then we can use our lcm function
    if numbers.len() == 2 {
        return lcm(numbers[0], numbers[1]);
    }
    // Otherwise we need to use recursion
    let head = numbers.first().unwrap().clone();
    let tail = &numbers[1..];
    return lcm(head, lcm_of_numbers(tail.to_vec()));
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
