use crate::ciphers;

pub struct RailFence {
    key: u8, // num of rail fences
}

impl RailFence {
    pub fn new(key: u8) -> Self {
        RailFence {
            key
        }
    }

    // find the start indexes of the rails in the ciphertext
    // TODO make simpler?
    fn rail_indexes(&self, len: usize) -> Vec<usize> {
        let loop_size = self.key as usize * 2 - 2;
        if len < loop_size {
            return (0..self.key as usize).collect();
        }

        let loops = len / loop_size; // round to zero
        let rem = len % (self.key as usize * 2 - 2);

        // find lengths of rails
        let mut rail_lens = vec![0; self.key as usize];
        rail_lens[0] = loops;
        // rail_lens[(self.key-1) as usize] = if rem > loop_size/2 {loops + 1} else {loops};
        for i in 1..self.key as usize - 1 {
            if rem > i {
                if rem > self.key as usize && rem - self.key as usize >= self.key as usize - i - 1 {
                    rail_lens[i] = 2*loops + 2;
                } else {
                    rail_lens[i] = 2*loops + 1;
                }
            } else {
                rail_lens[i] = loops
            }
        }

        // find indexes of rails in the encrypted output
        let mut output = vec![0; self.key as usize];
        output[0] = 0;
        for i in 1..self.key as usize {
            output[i] = output[i-1] + rail_lens[i-1];
        }
        return output;
    }
}

impl ciphers::Cipher for RailFence {
    fn encode(&self, pt: &Vec<u8>) -> Vec<u8> {
        let mut output = vec![0; pt.len()];
        let mut ixs = self.rail_indexes(pt.len());
        let mut rail_order = (0..self.key).chain(self.key-2..0).cycle();

        // put each byte in it's rail
        for b in pt {
            let rail = rail_order.next().unwrap() as usize;
            output[ixs[rail]] = *b;
            ixs[rail] += 1;
        }
        return output;
    }
    fn decode(&self, ct: &Vec<u8>) -> Vec<u8> {
        let mut output = vec![0; ct.len()];
        let mut ixs = self.rail_indexes(ct.len());
        let mut rail_order = (0..self.key).chain(self.key-2..0).cycle();

        // go through the rails in order and add the next byte for that rail to the output
        for i in 0..ct.len() {
            let rail = rail_order.next().unwrap() as usize;
            output[i] = ct[ixs[rail]];
            ixs[rail] += 1;
        }
        return output;
    }
}
