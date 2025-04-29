mod calculator;

use calculator::Calculator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <add|sub|mul|div> <a> <b>", args[0]);
        std::process::exit(1);
    }

    let op = &args[1];
    let a: i32 = args[2].parse().expect("Invalid number");
    let b: i32 = args[3].parse().expect("Invalid number");

    let result = match op.as_str() {
        "add" => Calculator::add(a, b),
        "sub" => Calculator::subtract(a, b),
        "mul" => Calculator::multiply(a, b),
        "div" => Calculator::divide(a, b),
        _ => {
            eprintln!("Unknown operation: {}", op);
            std::process::exit(1);
        }
    };

    println!("Result: {}", result);
}