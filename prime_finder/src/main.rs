// Use the num-primes crate for efficient prime checking
// use num_primes::Factorization; 
// use standard io library
use std::io;

// fn is_prime(n: u32) -> bool {
//     if n <= 1 {
//         return false;
//     }
//     if n == 2 || n == 3 {
//         return true; // 2 and 3 are prime numbers
//     }
//     if n % 2 == 0 {
//         return false; // eliminate even numbers
//     }
//     let sqrt_n = (n as f64).sqrt() as u32;
//     for i in (3..=sqrt_n).step_by(2) { // check odd numbers up to sqrt(n)
//         if n % i == 0 {
//             return false;
//         }
//     }
//     return true;
// }

fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_n = (n as f64).sqrt() as u32; // only need to check up to sqrt(n)
    for i in 2..=sqrt_n {
        if is_prime[i as usize] {
            primes.push(i); // add prime number to list
            let mut j = i * i; // start at i^2
            while j <= n { // mark multiples of i as not prime
                is_prime[j as usize] = false; // mark as not prime
                j += i; // increment by i
            }
        }
    }
    for i in sqrt_n + 1..=n {
        if is_prime[i as usize] {
            primes.push(i);
        }
    }
    return primes;
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to find all prime numbers up to that number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let upper_bound: u32 = input.trim().parse().expect("Please type a number!");
    
    // generate vector to store prime numbers
    let primes = sieve_of_eratosthenes(upper_bound);

    // find the number of prime numbers in the list

    let num_primes = primes.len();

    println!("Prime numbers up to {}: {:?}\nnumber of primes found {}", upper_bound, primes, num_primes);

}
