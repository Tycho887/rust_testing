// I want to make a function that checks if a number is prime or not




fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    // generate a list of numbers from 1 to 100
    let numbers = 1..101;   
    for n in numbers {
        if is_prime(n) {
            println!("{} is prime", n);
        }
    }
}