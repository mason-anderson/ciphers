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
    type Key  = u8;

    fn get_key(&self) -> Self::Key {
        self.key
    }

    fn encode(&self, pt: &ciphers::Plaintext) -> ciphers::Ciphertext {
        pt.iter().map(|b| u8::wrapping_add(*b, self.key)).collect()
    }

    fn decode(&self, pt: &ciphers::Ciphertext) -> ciphers::Plaintext {
        pt.iter().map(|b| u8::wrapping_sub(*b, self.key)).collect()
    }
}
