// This function uses .iter_mut() to iterate over a mutable reference to each element in a vector and increments each value by 1     
    
    fn main(){
        let mut v1 = vec![2,3,4,7,8,9];
     
        for val in v1.iter_mut(){
        
        *val = *val + 1;
        }

        println!("{:?}", v1);
        
    }