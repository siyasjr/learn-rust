struct Rect{
    height: u32,
    width: u32 
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width

    }

    fn perimeter(&self) -> u32{
        2 * (self.height + self.width)
    }

}

fn main(){
    let rect: Rect = Rect {
        height: 10,
        width: 20
    };

println!("The area of rect is {} sqcm" , rect.area());
println!("The perimeter of rect is {} cm" , rect.perimeter());


} // Implemented Struct 