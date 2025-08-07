 /*his program defines two traits: Summary and Fix, both with default implementations.

    The status function accepts only types that implement both traits.

    Random1 implements both, so it's accepted.

    Random2 does not implement Summary, so passing it to status causes a trait bound error. */
    
 
 pub trait Summary {
    fn summarize(&self) -> String{
        return format!("Summarized!!")
    }     
    }

pub trait Fix{
    fn fix(&self) -> String{
        return format!("Fixed!!")
    }     
    }

   

struct Random1 {}
struct Random2 {}

impl Summary for Random1{}
impl Fix for Random1{}

fn status<T: Summary + Fix> (item: T){        // The fn only accepts structs that impiments both Summary trait and Fix trait .
    println!("{}",item.summarize());
    println!("{}",item.fix());

}




fn main(){

    let random1 = Random1{};
    let random2 = Random2{};

    status(random1);
    // status(random2);  // Causes error : the trait bound `Random2: Summary` is not satisfied , the trait `Summary` is implemented for `Random1

}


// Out: Summarized!!
//      Fixed!!
