
struct Swap {
    first : i32,
    second: i32
}

pub trait  Serial {
    fn serialise(&self) -> Vec<u8>;

    
}

pub trait  Deserial:Sized {
    fn deserialise(v: &[u8]) -> Result<Self,Error>;
    
    
}



