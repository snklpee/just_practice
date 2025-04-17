fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum is {}",sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is {}",difference);

    // multiplication
    let product = 4 * 30;
    println!("product is {}",product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is {}",quotient);
    let truncated = -15.1 / 3.0; // Results in -1
    println!("truncated is {}",truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {}",remainder);

    // powers and exponentiation
    let exponent = 2.1f64;
    println!("exponentiation is: {}",exponent.powf(2.0));
}