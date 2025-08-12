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

impl Deserial for Swap {
    fn deserialise(v: &[u8]) -> Result<Self,Error>{
        if v.len() < 8 {
            return Err(Error);
        }

        let first_bytes:[u8 ;4] = v[0..4]
                                    .try_into()
                                    .map_err(|_| Error)?;

        let second_bytes:[u8 ; 4] = v[4..8]
                                        .try_into()
                                        .map_err(|_| Error)?;

        let first = i32::from_be_bytes(first_bytes);
        let second = i32::from_be_bytes(second_bytes);        
        Ok(Swap { first, second })

        


    }
}

fn main(){

    let s = Swap{
        first: 1025,
        second: 256
    };
    let ser = s.serialise();
    let des = Swap::deserialise(&ser).unwrap();

    println!("{:?}", ser);
    println!("{}, {}", des.first, des.second);
}