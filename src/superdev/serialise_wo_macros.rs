use::std::fmt::Error;

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

impl Serial for aSwap {
    fn serialise(&self) -> Vec<u8>{
     let mut result = Vec::new();
     result.extend_from_slice(&self.first.to_be_bytes());
     result.extend_from_slice(&self.second.to_be_bytes());
     result
    }

    
}

