pub trait Cipher {
    fn encode(&self, pt: &Vec<u8>) -> Vec<u8>;
    fn decode(&self, pt: &Vec<u8>) -> Vec<u8>;
    // fn from_key_str(key: &str) -> Result<Box<Self>, String> { Err("not implemented".to_string()) }
}
