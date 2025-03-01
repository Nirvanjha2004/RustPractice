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

fn main(){
    let mut string1 = String::from("Nirvan jha");
    let string2 = &mut string1;
    let string3 = &string1; // this wont work beacuse  string1 is already borrowed by an mutable reference string2

    println!("{}, {}", string2, string3);
}

//This works because 
fn main() {
    let mut str = String::from("Harkirat");
    let ref1 = &mut str;
    ref1.push_str("Singh");
    //Yahan ke baad uper wale ka scope khatam ho gya na use ho gya to...
    let ref2 = &str;
    
    println!("{}", ref2);
}