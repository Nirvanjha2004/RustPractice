/*
When you pass a variable by reference, the variable is still owned by the first function. It is only borrowed by the `get_length` function.

### Rules of borrowing -

1. You can only have one mutable reference. If there is an imutable reference, there cant be other immutable or mutable references
2. You can have multiple immutable references */

fn main(){
    let string1 = String::from("Nirvan Jha");
    let len = get_length(&string1);
    print!("{}", len);
}

fn get_length(str: &String)-> usize{
    return str.len();
}
