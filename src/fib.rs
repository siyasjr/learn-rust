fn main(){
    let num: u32 = 10;
    println!("{}", fib(num));
}

fn fib(n:u32)-> u32 {

    let mut first = 0; 
    let mut second = 1;

    if n == 0 {
        0
    }
    if n == 1 {
        1
    }

    for _ in 2..=n {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
