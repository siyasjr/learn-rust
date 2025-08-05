
// This program creates a HashMap from a vector of (String, i32) pairs and prints the full map and each key-value pair.

fn main(){
    let vec = vec![(String::from("Fayyaz"), 27), (String::from("Akmal"), 28), (String::from("Salam"), 29)];
    let mut hm = HashMap::new();
    for (key, val ) in vec{

        hm.insert(key, val);

    }

    println!("Full hashMap: {:?}", hm);
    for (key, val ) in hm{
        println!("Key:{}, val: {}", key, val);
    }

    
}