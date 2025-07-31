#[derive(Debug)]
enum Direction {
    North, 
    South,
    East,
    West
}

fn main(){
 let my_direction : Direction = Direction::North;
 println!("The current direction is {:?}", my_direction);
}




