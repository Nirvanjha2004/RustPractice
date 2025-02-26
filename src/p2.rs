

fn main () {
    let string1 : String = String::from("2");
    let num1 : u32 = 12;

    let string2 : String = num1.to_string();
    let num2 : Result<u32, std::num::ParseIntError> = string1.parse(); // Result<uint, String> does not work because .parse() internally calls and throws a defined error... ParseIntError

    match num2 {
        Ok(int_value) => println!("The parsed integer is {}", int_value),
        Err(_) => println!(" Threw an error")
    }

    println!("The results are {}", string2);
}
