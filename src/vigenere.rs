use crate::ciphers::*;

pub struct Vigenere {
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(key: Vec<u8>) -> Vigenere {
        Vigenere {
            key
        }
    }

    pub fn encode(&self, pt: &Plaintext) -> Ciphertext {
        let mut pos: usize = 0;
        let mut output: Ciphertext = Vec::new();
        for b in pt {
            output.push(u8::wrapping_add(*b, self.key[pos]));
            pos = (pos + 1) % self.key.len();
        }
        return output;
    }

    pub fn decode(&self, pt: &Ciphertext) -> Plaintext {
        let mut pos: usize = 0;
        let mut output: Ciphertext = Vec::new();
        for b in pt {
            output.push(u8::wrapping_sub(*b, self.key[pos]));
            pos = (pos + 1) % self.key.len();
        }
        return output;
    }
}


