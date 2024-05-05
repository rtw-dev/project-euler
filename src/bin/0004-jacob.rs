// https://projecteuler.net/problem=4
// Find the largest palindrome made from the product of two 3-digit numbers.
// Answer: 906609

fn main() {
    /*
    Strategy: Consider all the possible products and find the maximum palinromic product.
    */

    // Define a way to store the palindromes we find
    struct Palindrome {
        i: u32,
        j: u32,
        product: u32,
    }
    let mut palindromes: Vec<Palindrome> = Vec::new();

    // Consider all the possible products and store the palindromes we find
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) {
                palindromes.push(Palindrome { i, j, product });
            }
        }
    }

    // Determine the largest palindrome
    let biggest_product: Palindrome = palindromes.into_iter().max_by_key(|p| p.product).unwrap();
    println!(
        "Simple solution: {0} = {1} * {2}",
        biggest_product.product, biggest_product.i, biggest_product.j
    );
}

fn is_palindrome(number: u32) -> bool {
    /*
    Strategy: convert the number to a string, reverse it,
    and then check if the reversed string is equal to the original string.
    */
    let number_str = number.to_string();
    if number_str == number_str.chars().rev().collect::<String>() {
        return true;
    }
    return false;
}
