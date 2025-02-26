
struct TwoNumbers{
    a : u32,
    b : u32
}

impl TwoNumbers {
    fn add(&self) -> u32{
        return self.a+self.b;
    }
    fn subtract(&self)->u32{
        return self.a-self.b;
    }
    fn multiply(&self)-> u32{
        return self.a*self.b;
    }
    fn divide(&self)-> u32{
        if self.a!=0 && self.b!=0{
            return self.a/self.b;
        } else {
            return 0;
        }
    }
}
fn main () {
    let myfunction : TwoNumbers = TwoNumbers{
        a : 20,
        b : 12
    };

    let sub_val  : u32 = myfunction.subtract();
    let divide_val: u32 = myfunction.divide();
    let mul_val : u32 = myfunction.multiply();
    let add_val: u32 = myfunction.add();

    println!("The values are {},{},{},{}", sub_val, add_val, mul_val, divide_val);

}
