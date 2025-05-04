use crate::ciphers;

pub struct Ceaser {
    key: u8
}

impl Ceaser {
    pub fn new(key: u8) -> Ceaser {
        Ceaser{key}
    }
}

impl ciphers::Cipher for Ceaser {
    fn encode(&self, pt: &Vec<u8>) -> Vec<u8> {
        pt.iter().map(|b| u8::wrapping_add(*b, self.key)).collect()
    }

    fn decode(&self, pt: &Vec<u8>) -> Vec<u8> {
        pt.iter().map(|b| u8::wrapping_sub(*b, self.key)).collect()
    }
}
