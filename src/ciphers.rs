//#[derive(Debug, PartialEq)]
//pub struct Plaintext(pub Vec<u8>);
//#[derive(Debug, PartialEq)]
//pub struct Ciphertext(pub Vec<u8>);

pub type Plaintext = Vec<u8>;
pub type Ciphertext = Vec<u8>;

pub trait Cipher {
    type Key;
    fn get_key(&self) -> Self::Key;
    fn encode(&self, pt: &Plaintext) -> Ciphertext;
    fn decode(&self, pt: &Ciphertext) -> Plaintext;
}
