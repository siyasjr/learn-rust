
// This function filters even numbers from a vector using .iter() and .filter(), prints them, and demonstrates that .iter() can be reused since it's only borrowed immutably.

fn main(){
    let v1 = vec![1,2,3,4,5,6];  // no need to be mut because iter() borrows it as ref and also .map borrows iter() as ref
    let iter = v1.iter();
    let iter2 = iter.filter(|x| *x % 2 ==0);
    for x in iter2 {
        println!("{}" , x);
    }

    let iter3 = iter; // Still can access v1.iter afterwards
}
