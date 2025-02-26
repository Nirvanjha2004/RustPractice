fn main () {
    let mut i : u32 = 0;
    let var : u32 = 10;
    let mut sum : u32 = 0;
    while i <= var {
        sum = sum + i;
        i = i+1;
    }
    print!("The sum while using while loop is {}", sum);
    sum = 0;
    //i = 0; for loop jo hai uses the concept of shadowing means it copies the var i means its a new var in for loop ... and if for ki jagah while hi use krte to it does not support shadowing... so tb ye vapas i =0 krna padta..

    for i in 0..var {
        sum = sum + i;
    }

    print!("The sum according to For loop is {}", sum);

}
