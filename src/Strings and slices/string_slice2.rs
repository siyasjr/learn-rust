/* This program extracts the first and last names from a full name string.

    get_first_name returns the part before the first space.

    get_last_name returns the part after the first space (if any), trimming leading spaces and returning None if empty.  */

fn main(){
    let ogstr = String::from("Akash Madhav");
    let ans1 = get_first_name(&ogstr);
     
    println!("The first name is {}",ans1 );

    if let Some(last) = get_last_name(&ogstr){
        println!("The last name is {}", last);
    }
    else {
        println!("No last name found")
    }
   
}

fn get_first_name(name: &str) -> &str {
    for (i, val) in name.chars().enumerate(){
        if val == ' ' {
            return &name[0..i];
        }
    } 
    return &name[..];
}


fn get_last_name(name: &str) -> Option<&str> {

    for (i, val) in name.chars().enumerate(){
        if val == ' '{
            let trimmed = name[i+1 ..].trim_start();
            if trimmed.is_empty(){
                return None;
            }
            else {
                return Some(trimmed);
            }
           
        }

    }
    None 
}