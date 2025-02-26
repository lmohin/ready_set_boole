mod adder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error\n Wrong number of arguments {}", args.len());
        std::process::exit(1);
    }
    let a: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error\n '{}' is not a valid integer", args[1]);
            std::process::exit(1);
        }
    };
    let b: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error\n '{}' is not a valid integer", args[2]);
            std::process::exit(1);
        }
    };
    println!("{} + {} = {}", a, b, adder::adder(a, b));
    println!("{} + {} = {}", -42, 42, adder::int_adder(-42, 42));
    println!("{} + {} = {}", -42, -42, adder::int_adder(-42, -42));
    println!("{} + {} = {}", -2147483648, -1, adder::int_adder(-2147483648, -1));
}
