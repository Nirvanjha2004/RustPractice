//ENUMS
enum Direction{
    North,
    East,
    South,
    West
}
fn main(){
    let direction = Direction::North;
    let new_direction = direction; // No error because direction is copy

    move_around(new_direction);
}

fn move_around(direction: Direction){
    println!("{}", direction); //Right now throws an error because it does not have display trait
}