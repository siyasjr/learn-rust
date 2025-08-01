
//pattern matched enum 
/*enum Shape {
    Circle(f64), 
    Sqaure(f64),
    Rectangle(f64, f64),
}

fn main(){

    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Sqaure(10.0);
    let rectangle: Shape = Shape::Rectangle(10.0,5.0);


    println!("The area of circle is {}", calculate_area(circle));
    println!("The area of square is {}", calculate_area(square));
    println!("The area of rectangle is {}", calculate_area(rectangle));
}

fn calculate_area(shape: Shape) -> f64 {

    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Sqaure(side) => side  *side,
        Shape:: Rectangle(width , height) => width * height,
    }                        // no ; , implicit return  
}*/ 

// Updated with debug trait
#[derive(Debug)]

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main(){
    let circle: Shape = Shape::Circle(15.0);
    let rect: Shape = Shape::Rectangle(15.0, 20.0);
    let square: Shape = Shape::Square(25.0);

    println!("The area of the {:?} is {} ", circle,calc_area(&circle) );
    println!("The area of the {:?} is {} ",rect,calc_area(&rect) );
    println!("The area of the {:?} is {} ",square,calc_area(&square) );
}

fn calc_area(shape: &Shape) -> f64 {
   match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(height, width) => height * width,
        Shape::Square(side) => side * side,
        
    }
    

}
