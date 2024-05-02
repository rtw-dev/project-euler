// https://projecteuler.net/problem=5
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

const BOUND: i32 = 20;

fn main() {
    /*
    Strategy: Use known mathematical facts about Least Common Multiples (LCFs) and Greatest Common Divisors (GCDs):
    - LCM(a, b) = (a * b) / GCD(a, b))
    - GCD(a, b) = GCD(b, a mod b) - the Euclidean Algorithm
    - LCM(a, b, c) = LCM(a, LCM(b,c)) - so we can use a recursive approach
    */

    // Recursive approach is easy to understand
    let numbers: Vec<i32> = (0..BOUND).collect();
    let solution_by_recursion = lcm_of_numbers(numbers);
    println!("Simple solution: {}", solution_by_recursion);

    // Folding solution requires a bit more thought
    let solution_by_folding = (1..BOUND).fold(1, |acc, n| (acc * n) / gcd(acc, n));
    println!("Better solution: {}", solution_by_folding);
}

fn lcm_of_numbers(numbers: Vec<i32>) -> i32 {
    // If only two numbers in the list then we can use our lcm function
    if numbers.len() == 2 {
        return lcm(numbers[0], numbers[1]);
    }
    // Otherwise we need to use recursion
    let (head, tail) = numbers.split_at(1);
    return lcm(*head, lcm_of_numbers(tail));
}

fn lcm(a: i32, b: i32) -> i32 {
    return a * b / gcd(a, b);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
