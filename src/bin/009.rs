// https://projecteuler.net/problem=9
// Find a*b*c such that a^2 + b^2 = c^2 and a + b + c = 1000
// Answer: 31875000

const BOUND: u32 = 1000;

fn main() {
    for a in 1..BOUND {
        for b in 1..BOUND {
            for c in 1..BOUND {
                if a * a + b * b == c * c && a + b + c == BOUND {
                    println!("Solution: {} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}
