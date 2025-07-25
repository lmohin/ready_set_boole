mod adder;
mod multiplier;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error\nPlease provide two arguments");
        std::process::exit(1);
    }
    let a: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error\n'{}' is not a valid positive integer", args[1]);
            std::process::exit(1);
        }
    };
    let b: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error\n'{}' is not a valid positive integer", args[2]);
            std::process::exit(1);
        }
    };
    println!("{} * {} = {}", a, b, multiplier::multiplier(a, b));
    println!("{} * {} = {}", -42, 42, multiplier::int_multiplier(-42, 42));
    println!("{} * {} = {}", 42, -42, multiplier::int_multiplier(42, -42));
    println!("{} * {} = {}", -42, -42, multiplier::int_multiplier(-42, -42));
    println!("{} * {} = {}", -2147483648, -1, multiplier::int_multiplier(-2147483648, -1));
}
