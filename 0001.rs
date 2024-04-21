fn main() {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 {
            // println!("{}", n);
            sum = sum + n;
        } else if n % 5 == 0 {
            //println!("{}", n);
            sum = sum + n;
        }
        println!("{}", sum);
    }
}}