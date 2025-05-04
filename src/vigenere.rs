use crate::ciphers;

pub struct Vigenere {
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(key: Vec<u8>) -> Vigenere {
        Vigenere {
            key
        }
    }

}

impl ciphers::Cipher for Vigenere {
    fn encode(&self, pt: &Vec<u8>) -> Vec<u8> {
        let mut pos: usize = 0;
        let mut output: Vec<u8> = Vec::new();
        for b in pt {
            output.push(u8::wrapping_add(*b, self.key[pos]));
            pos = (pos + 1) % self.key.len();
        }
        return output;
    }

    fn decode(&self, pt: &Vec<u8>) -> Vec<u8> {
        let mut pos: usize = 0;
        let mut output: Vec<u8> = Vec::new();
        for b in pt {
            output.push(u8::wrapping_sub(*b, self.key[pos]));
            pos = (pos + 1) % self.key.len();
        }
        return output;
    }
}
