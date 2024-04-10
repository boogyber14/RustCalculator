use std::io;

fn main() {
    println!("Welcome to Simple Rust Calculator");
    println!("Enter two numbers separated by space:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    if numbers.len() !=2 {
        println!("Please enter exactly two numbers separated by space.");
        return;
    }

    let result = numbers[0] * numbers[1]; // Change to subtraction
    println!("Result: {}", result);
}