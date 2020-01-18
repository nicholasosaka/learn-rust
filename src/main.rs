use std::io::Write;
mod sieve;

fn main() {
    let mut limit= String::new();

    print!("Enter the limit for the sieve: ");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut limit)
        .expect("Failed to get input");

    let limit : i32 = limit.trim().parse()
        .expect("Failed to convert to i32");

    let primes = sieve::primes(limit);

    for p in primes {
        print!("{} ", p);
    }
}
