use crate::ciphers;

use crate::caesar;
use crate::vigenere;
use crate::enigma;

pub enum CipherTypes {
    Caesar(caesar::Ceaser),
    Vigenere(vigenere::Vigenere),
    Enigma(enigma::Enigma),
}

pub struct Combination {
    ciphers: Vec<CipherTypes>
}

impl Combination {
    pub fn new(key: Vec<CipherTypes>) -> Self {
        Combination {
            ciphers: key,
        }
    }
}

impl ciphers::Cipher for Combination {
    fn encode(&self, pt: &Vec<u8>) -> Vec<u8> {
        let mut input: Vec<u8>;
        let mut output: Vec<u8> = pt.to_owned();
        for cipher in &self.ciphers {
            input = output;
            match cipher {
                CipherTypes::Caesar(c) => {
                    output = c.encode(&input);
                }
                CipherTypes::Vigenere(c) => {
                    output = c.encode(&input);
                }
                CipherTypes::Enigma(c) => {
                    output = c.encode(&input);
                }
            }
        }
        return output;
    }
    fn decode(&self, ct: &Vec<u8>) -> Vec<u8> {
        let mut input: Vec<u8>;
        let mut output: Vec<u8> = ct.to_owned();
        for cipher in self.ciphers.iter().rev() {
            input = output;
            match cipher {
                CipherTypes::Caesar(c) => {
                    output = c.decode(&input);
                }
                CipherTypes::Vigenere(c) => {
                    output = c.decode(&input);
                }
                CipherTypes::Enigma(c) => {
                    output = c.decode(&input);
                }
            }
        }
        return output;
    }
}
