use crate::ChaCha20;

pub struct ChaChaRng {
    cipher: ChaCha20::ChaChaCipher,
    seed: [u8;32],
    nonce: [u8;12],
}

impl ChaChaRng {
    pub fn new(seed: [u8;32], nonce: [u8;12]) -> ChaChaRng {
        ChaChaRng {
            cipher: ChaCha20::ChaChaCipher::new(seed, 0, nonce),
            seed: seed,
            nonce: nonce,
        }
    }

    fn bytesToU32(data:[u8;4]) -> u32 {
        ((data[0] as u32 & 0xFF))  |
        ((data[1] as u32 & 0xFF) <<  8) |
        ((data[2] as u32 & 0xFF) << 16) |
        ((data[3] as u32 & 0xFF) << 24)
    }

    pub fn getNextInt(self: &mut ChaChaRng) -> u8 {
        // let mut output = [0;4];
        let output = self.cipher.update1([0]);
        // ChaChaRng::bytesToU32(output)
        output[0]
    }
}

// #[test]
// fn test_ChaChaRng_1() {
//     let key  = [
//         0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
//         0, 0, 0
//     ];

//     let nonce = [
//         0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00
//     ];

//     let mut rng = ChaChaRng::new(key, nonce);

//     assert_eq(rng.getNextInt(), 0x082d9b72);
// }

// #[test]
// fn test_ChaChaRng_nonce() {
//     let key  = [0; 32];

//     let nonce = [
//         0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0,
//         0, 2
//     ];

//     let mut rng20 = ChaChaRng::new(key, nonce);

//     let mut result = [0; 20];

//     for i in 0..16 {
//         result[i] = rng20.getNextInt();
//     }

//     let expected = [
//         0x374dc6c2, 0x3736d58c, 0xb904e24a, 0xcd3f93ef,
//         0x88228b1a, 0x96a4dfb3, 0x5b76ab72, 0xc727ee54,
//         0x0e0e978a, 0xf3145c95, 0x1b748ea8, 0xf786c297,
//         0x99c28f5f, 0x628314e8, 0x398a19fa, 0x6ded1b53
//     ];

//     for i in 0..16 {
//         assert_eq(result[i], expected[i]);
//     }
// }