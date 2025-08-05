/*  key value pairs . Similar to dict in Python and objects in JS
 Methods : 
    1. insert,
    2. get,
    3. remove,
    4. clear
*/

use std::collections::HashMap;

fn main(){

    let mut hm = HashMap::new();

    hm.insert(String::from("Ajmal"),27 );

    let usr = hm.get("Ajmal");
    match usr {
        Some(data) => {println!("{}", data) },

        None => println!("There is no data in the DB")       
    }
    
    println!("{:?}", hm)
   
}