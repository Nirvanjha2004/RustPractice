// Pattern Matching
enum Shapes{
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64)
}

fn get_area(shape: Shapes)->f64{
    match shape {
        Shapes:: Circle(radius) => std::f64::consts::PI*radius*radius,
        Shapes:: Rectangle(length, breadth) => length*breadth,
        Shapes:: Square(side) => side*side
    }
}

fn main(){

    let circle = Shapes:: Circle(5.0);
    let rect = Shapes:: Rectangle(5.0, 6.0);
    let square = Shapes:: Square(4.0);

    println!("The are of all the three are {}, {}, {}", get_area(circle),get_area(rect),get_area(square));
}