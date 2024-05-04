// https://projecteuler.net/problem=6
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

const BOUND: i64 = 100;

fn main() {
    println!("Simple solution: {}", simple_solution());
    println!("Better solution: {}", better_solution());
}

fn simple_solution() -> i64 {
    let mut sum_of_squares: i64 = 0;
    let mut sum: i64 = 0;
    for i in 1..=BOUND {
        sum_of_squares += i * i;
        sum += i;
    }
    let square_of_sum = sum * sum;
    return square_of_sum - sum_of_squares;
}

fn better_solution() -> i64 {
    let sum_of_squares: i64 = (1..=BOUND).map(|n| n * n).sum();
    let sum: i64 = (1..=BOUND).sum();
    let square_of_sum = sum * sum;
    return square_of_sum - sum_of_squares;
}
