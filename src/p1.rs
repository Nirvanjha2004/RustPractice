use std::io;
fn sum(a: u32, b:u32) -> u32 {
    return a + b;
}

fn main () {
    println!("Enter the first input:");
    let mut input1: String = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num1: u32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the second number:");
    let mut input2: String = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read the second input");
    let num2: u32 = input2.trim().parse().expect("Enter a valid 2nd number");

    let result: u32= sum(num1, num2);
    println!("The sum of a and b is {}", result);
}
