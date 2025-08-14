use std::fmt::Error;

trait Serialize {
    // Serializes qty_1 and qty_2 into a bunch of bytes
    // if struct looks like this = Swap {qty_1: 1, qty_2: 2}
    // output looks like [0, 0, 0, 1, 0, 0, 0, 2]
	fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: &[u8]) -> Result<Swap, std::fmt::Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32
}
impl Deserialize for Swap {
    fn deserialize(data: &[u8]) -> Result<Self, std::fmt::Error> {
        if data.len() < 8 {
            return Err(std::fmt::Error);
        }
        let qty_1 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        return Ok(Swap { 
            qty_1, 
            qty_2 
        });
    }
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        // TODO
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

fn main() {
    let s = Swap {
        qty_1: 1,
        qty_2: 2
    };

    let v = s.serialize();
    print!("{:?}", v);

    let s2 = Swap::deserialize(&v).unwrap();
    println!("{:?}", s2);
}