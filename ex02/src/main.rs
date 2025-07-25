mod gray_coder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error\nPlease provide one argument");
        std::process::exit(1);
    }
    let n: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error\n'{}' is not a valid positive integer", args[1]);
            std::process::exit(1);
        }
    };
    println!("Gray code equivalent of {} is: {}", n, gray_coder::gray_code(n));
}
