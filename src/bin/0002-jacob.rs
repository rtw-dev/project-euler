fn main() {
    /*
    Find the sum of the even-valued Fibonnaci numbers below four million.
    */

    // We will use three variables to store the last two and the next Fibonnaci numbers.
    let mut prev1 = 1;
    let mut prev2 = 2;
    let mut next;

    // We will start the sum by including the second Fibonnaci number.
    let mut sum = 2;

    loop {
        // Calculate the next Fibonacci number
        next = prev1 + prev2;
        // If the next Fibonacci number is greater than 4 million, break out of our loop
        if next > 4000000 {
            break;
        }
        // If the next Fibonacci number is even, add it to the sum
        if next % 2 == 0 {
            sum = sum + next;
        }
        // Shuffle the variables
        prev1 = prev2;
        prev2 = next;
    }
    println!("{}", sum);
}
