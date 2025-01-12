use crate::ciphers::*;

type Rotor = [u8; 256];

pub const NUM_ROTORS: usize = 5;

static ROTORS: [Rotor; NUM_ROTORS] = [
    [37, 212, 39, 235, 173, 133, 141, 80, 118, 4, 72, 25, 17, 106, 33, 226, 68, 52, 181, 163, 28, 79, 114, 60, 93, 89, 40, 201, 91, 87, 41, 34, 250, 199, 59, 126, 8, 144, 209, 148, 213, 6, 32, 98, 99, 67, 10, 77, 64, 42, 30, 31, 101, 165, 21, 97, 27, 145, 198, 116, 44, 251, 84, 74, 82, 92, 151, 96, 122, 132, 175, 248, 240, 121, 104, 75, 100, 51, 69, 20, 65, 12, 7, 150, 73, 217, 49, 127, 102, 57, 187, 185, 193, 228, 109, 224, 255, 43, 131, 3, 9, 108, 149, 184, 169, 18, 227, 13, 205, 94, 174, 53, 1, 0, 139, 70, 50, 164, 245, 29, 252, 229, 110, 47, 157, 254, 202, 176, 35, 88, 26, 203, 221, 242, 156, 115, 81, 253, 192, 161, 90, 124, 186, 112, 236, 188, 158, 223, 62, 103, 146, 243, 36, 78, 194, 159, 152, 61, 177, 208, 24, 220, 247, 119, 86, 111, 19, 249, 178, 206, 238, 183, 197, 142, 196, 130, 14, 210, 219, 134, 138, 168, 46, 241, 38, 56, 55, 95, 189, 200, 137, 155, 22, 239, 147, 85, 123, 76, 125, 128, 153, 246, 171, 222, 136, 154, 244, 180, 190, 107, 83, 58, 234, 117, 191, 170, 113, 135, 167, 2, 232, 140, 166, 172, 214, 195, 54, 23, 237, 15, 204, 129, 233, 231, 211, 225, 11, 71, 48, 218, 207, 179, 63, 66, 162, 45, 230, 120, 216, 16, 160, 105, 182, 215, 5, 143],
    [16, 193, 214, 254, 226, 250, 179, 24, 218, 131, 136, 142, 128, 175, 150, 17, 65, 45, 139, 30, 41, 66, 7, 161, 203, 37, 187, 148, 189, 144, 59, 196, 93, 72, 121, 43, 20, 29, 242, 119, 191, 62, 87, 77, 213, 96, 95, 225, 207, 206, 156, 209, 104, 251, 53, 245, 101, 70, 152, 117, 55, 184, 73, 253, 168, 134, 162, 109, 90, 85, 80, 8, 216, 241, 186, 235, 157, 211, 42, 49, 129, 194, 167, 6, 102, 197, 171, 112, 18, 63, 159, 182, 195, 158, 107, 28, 68, 145, 147, 50, 217, 208, 91, 57, 64, 199, 178, 60, 219, 52, 188, 15, 180, 133, 233, 220, 1, 124, 111, 114, 155, 61, 56, 115, 137, 47, 39, 75, 14, 94, 240, 34, 183, 127, 190, 48, 224, 4, 166, 239, 38, 21, 228, 83, 143, 229, 172, 212, 140, 185, 234, 113, 9, 237, 135, 238, 54, 201, 99, 110, 78, 67, 164, 58, 88, 86, 151, 79, 221, 10, 169, 25, 141, 3, 13, 130, 153, 126, 252, 33, 202, 100, 215, 165, 205, 210, 36, 23, 11, 255, 0, 69, 181, 108, 44, 173, 123, 26, 5, 40, 76, 244, 249, 51, 71, 146, 154, 22, 248, 120, 106, 89, 174, 176, 81, 31, 230, 12, 163, 82, 116, 198, 84, 222, 32, 74, 46, 243, 118, 223, 122, 246, 92, 97, 160, 149, 105, 19, 247, 35, 103, 200, 231, 132, 2, 227, 27, 177, 138, 204, 236, 192, 170, 232, 125, 98],
    [51, 66, 202, 213, 78, 216, 246, 32, 104, 79, 207, 166, 172, 129, 109, 150, 68, 137, 14, 250, 99, 16, 52, 170, 198, 184, 43, 19, 81, 82, 59, 204, 173, 242, 48, 28, 238, 130, 219, 195, 36, 188, 33, 115, 237, 18, 86, 37, 90, 91, 147, 145, 0, 44, 191, 64, 218, 125, 200, 177, 225, 254, 110, 27, 10, 189, 210, 72, 85, 101, 45, 123, 157, 102, 136, 223, 114, 131, 220, 65, 217, 236, 206, 3, 146, 76, 135, 199, 75, 255, 70, 25, 197, 49, 175, 5, 186, 182, 103, 63, 105, 153, 231, 149, 54, 106, 122, 23, 148, 12, 128, 241, 190, 22, 162, 165, 116, 34, 117, 29, 119, 42, 226, 228, 120, 2, 152, 252, 6, 89, 160, 57, 205, 121, 158, 30, 233, 159, 141, 126, 80, 87, 31, 107, 133, 15, 96, 209, 77, 26, 100, 164, 251, 214, 98, 227, 142, 178, 193, 163, 127, 235, 53, 208, 134, 192, 11, 97, 243, 17, 92, 174, 47, 95, 108, 185, 124, 84, 187, 83, 50, 113, 62, 239, 132, 245, 13, 221, 39, 9, 144, 248, 244, 171, 140, 21, 143, 253, 41, 168, 138, 61, 55, 94, 40, 234, 67, 112, 7, 155, 20, 71, 222, 73, 58, 203, 181, 240, 212, 167, 215, 118, 224, 56, 249, 35, 60, 46, 176, 38, 230, 154, 111, 139, 69, 169, 183, 1, 8, 24, 4, 161, 232, 74, 93, 180, 211, 194, 179, 156, 196, 151, 247, 201, 229, 88],
    [71, 250, 144, 118, 239, 39, 29, 196, 236, 136, 35, 195, 224, 231, 68, 17, 148, 95, 158, 130, 186, 191, 165, 101, 161, 217, 21, 190, 234, 54, 204, 179, 91, 180, 104, 162, 37, 249, 153, 87, 79, 147, 4, 94, 82, 194, 245, 30, 235, 169, 111, 48, 121, 1, 59, 43, 248, 221, 222, 78, 52, 13, 203, 160, 10, 229, 18, 120, 112, 86, 232, 42, 100, 97, 32, 228, 238, 61, 19, 156, 49, 140, 159, 168, 12, 90, 209, 69, 123, 218, 8, 127, 103, 124, 75, 233, 63, 220, 122, 98, 28, 206, 139, 151, 244, 192, 56, 6, 157, 15, 24, 45, 213, 26, 170, 137, 212, 199, 133, 81, 178, 119, 14, 85, 92, 208, 183, 227, 177, 128, 5, 110, 3, 202, 132, 88, 164, 215, 138, 200, 167, 46, 216, 125, 149, 126, 207, 225, 246, 189, 57, 182, 154, 174, 105, 155, 89, 255, 38, 252, 242, 70, 219, 175, 106, 99, 197, 83, 72, 150, 58, 84, 76, 173, 210, 166, 129, 41, 44, 205, 51, 60, 16, 240, 193, 25, 65, 80, 115, 237, 67, 20, 109, 55, 152, 143, 185, 40, 198, 66, 73, 2, 47, 107, 96, 176, 211, 108, 135, 188, 114, 230, 7, 36, 247, 184, 142, 27, 134, 102, 74, 31, 172, 33, 226, 113, 23, 201, 64, 214, 241, 187, 11, 243, 9, 131, 171, 163, 181, 251, 93, 116, 34, 223, 77, 145, 22, 50, 0, 253, 53, 254, 117, 141, 146, 62],
    [186, 194, 159, 98, 162, 39, 150, 242, 174, 70, 6, 93, 212, 171, 42, 79, 147, 126, 173, 246, 249, 17, 217, 90, 19, 160, 224, 163, 7, 100, 101, 189, 102, 87, 181, 250, 155, 253, 96, 78, 57, 80, 54, 185, 112, 143, 178, 251, 238, 95, 237, 172, 214, 145, 166, 255, 231, 118, 122, 69, 66, 138, 211, 127, 83, 222, 63, 10, 71, 165, 168, 43, 208, 201, 193, 67, 131, 190, 195, 137, 169, 104, 94, 31, 170, 1, 35, 219, 3, 105, 91, 60, 11, 20, 218, 205, 85, 14, 230, 233, 129, 161, 76, 132, 32, 124, 27, 89, 65, 223, 21, 243, 4, 40, 44, 180, 73, 53, 136, 158, 191, 109, 33, 152, 59, 142, 82, 141, 210, 134, 207, 50, 97, 120, 61, 24, 62, 197, 146, 12, 25, 49, 254, 99, 234, 34, 121, 107, 117, 215, 8, 16, 51, 236, 123, 0, 144, 203, 56, 36, 196, 188, 68, 213, 77, 245, 84, 135, 111, 92, 37, 75, 200, 110, 182, 48, 74, 41, 148, 139, 252, 47, 45, 199, 154, 198, 72, 9, 175, 125, 108, 179, 133, 206, 2, 29, 221, 30, 202, 227, 192, 151, 38, 116, 23, 220, 58, 248, 106, 88, 13, 46, 167, 216, 156, 52, 149, 5, 183, 228, 239, 157, 235, 114, 164, 226, 204, 153, 18, 22, 103, 128, 119, 176, 113, 209, 26, 130, 240, 177, 81, 64, 232, 244, 229, 184, 241, 247, 115, 15, 86, 140, 55, 225, 187, 28],
];

static INVERSE_ROTORS: [Rotor; NUM_ROTORS] =
    [inverse_rotor(&ROTORS[0]), inverse_rotor(&ROTORS[1]), inverse_rotor(&ROTORS[2]), inverse_rotor(&ROTORS[3]), inverse_rotor(&ROTORS[4])];

const fn inverse_rotor(rotor: &Rotor) -> Rotor {
    let mut inverse: Rotor = [0; 256];
    let mut i = 0;
    while i < 256 {
        inverse[rotor[i] as usize] = i as u8;
        i += 1;
    }
    return inverse;
}

pub struct Enigma {
    rotors: Vec<usize>,
    rotor_poses: Vec<u8>,
    plugboard: [u8; 256],
}

fn advance_rotor_poses(poses: &mut Vec<u8>) {
    for i in 0..poses.len() {
        if poses[i] < 255 {
            poses[i] += 1;
            return;
        }
        poses[i] = 0;
    }
}

impl Enigma {
    pub fn new(rotors: Vec<usize>, rotor_poses: Vec<u8>, plugboard: &[u8; 256]) -> Result<Enigma, String> {
        for rotor in &rotors {
            if *rotor >= NUM_ROTORS {
                return Err(format!("invalid rotor number: {}", rotor))
            }
        }
        if rotors.len() != rotor_poses.len() {
            return Err("number of rotors does not match number of initial positions".to_string());
        }
        for i in 0..plugboard.len() {
            if plugboard[plugboard[i] as usize] as usize != i {
                return Err("plugboard must map in pairs (if a maps to b then b must map to a)".to_string());
            }
        }

        return Ok(Enigma {
            rotors,
            rotor_poses,
            plugboard: *plugboard,
        });
    }

    fn encode_byte(&self, cur_poses: &Vec<u8>, byte: u8) -> u8 {
        let mut byte = self.plugboard[byte as usize];

        // go through rotors forward
        for i in 0..self.rotors.len() {
            let input: usize = u8::wrapping_add(byte, cur_poses[i]) as usize;
            byte = u8::wrapping_sub(ROTORS[self.rotors[i]][input], cur_poses[i]);
        }
        // reflector
        // TODO use a random but fixed reflector?
        byte = u8::wrapping_add(byte, 128);
        // go through rotors backwards
        for i in (0..self.rotors.len()).rev() {
            let input: usize = u8::wrapping_add(byte, cur_poses[i]) as usize;
            byte = u8::wrapping_sub(INVERSE_ROTORS[self.rotors[i]][input], cur_poses[i]);
        }

        byte = self.plugboard[byte as usize];
        return byte;
    }

    pub fn encode(&self, pt: &Plaintext) -> Ciphertext {
        let mut rotor_poses = self.rotor_poses.clone();

        let mut ct: Ciphertext = Vec::new();
        for byte in pt {
            ct.push(self.encode_byte(&rotor_poses, *byte));
            advance_rotor_poses(&mut rotor_poses);
        }
        return ct;
    }

    pub fn decode(&self, pt: &Ciphertext) -> Plaintext {
        return self.encode(pt);
    }
}


