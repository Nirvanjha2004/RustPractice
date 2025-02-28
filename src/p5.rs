fn main() {
    let string1 = String::from("Nirvan Jha");
    let (strlength, str) = get_length(string1);
    println!("{}", strlength);
    println!("{}", str);
    //print!("{}", string1); Here the ownership is transferred to get_length function and gets destroyed when the function gets out of scope.... so we have to transfer ownership again

}

fn get_length(str: String)-> (usize, String){
    return (str.len(), str); // This is how we are transferring ownership 
}
