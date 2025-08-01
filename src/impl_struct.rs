#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn calc_area(&self)-> u32{
        self.height * self.width
    }

    fn calc_perimeter(&self)-> u32{
        2 * (self.width + self.height)
    }

}

fn main(){
    let rect: Rect = Rect {
        height: 10,
        width: 20,

    };
    println!("The area of the the {:?} is {} sqcm" ,rect, rect.calc_area());
    println!("The perimeter of the {:?} is {} cm" ,rect, rect.calc_perimeter());
   
}
