fn sum(a: u32, b:u32) -> u32 {
    return a + b;
}

fn main() {
    let a = 5;
    let b = 10;
    let result = sum(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
}
