
struct Swap {
    first : i32,
    second: i32
}

pub trait  Serial {
    fn serialise(&self) -> Vec<u8>;

    
}

