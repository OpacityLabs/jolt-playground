use crate::ChaCha20Rng::ChaChaRng;

fn u128_to_bits_le(mut num: u128) -> [u32; 128] {
    let mut bits = [0; 128];
    let mut index = 0;

    while num > 0 {
        bits[index] = (num & 1) as u32;
        num >>= 1;
        index += 1;
    }

    bits
}

pub struct ChaChaEncoder {
    key: [u8;32],
}

// struct EncodingLabel {
//     labels: [[u8;16];8]
// }

// { labels: [ 8, 16 ], delta: 16 }

impl ChaChaEncoder {
    pub fn new(key: [u8;32]) -> ChaChaEncoder {
        ChaChaEncoder {
            key,
          //  delta: [13, 185, 152, 249, 169, 155, 162, 86, 133, 244, 64, 147, 124, 219, 175, 56]
        }
    }


    pub fn get_encodings(self: &mut ChaChaEncoder, id: [u8;12])-> [[u8;16];8] {

        let mut labels = [[0;16];8];

        let mut rng = ChaChaRng::new(self.key,id);


        for i in 0..8 {
            let mut label:[u8;16] = [0;16];

            for j in 0..label.len() {
                label[j] = rng.getNextInt();
            }
            labels[i] = label;
        }


        labels
    }

    fn bytesToU642(data:[u8;16]) -> u64 {
        ((data[15] as u64 & 0xFF))  |
        ((data[14] as u64 & 0xFF) <<  8) |
        ((data[13] as u64 & 0xFF) << 16) |
        ((data[12] as u64 & 0xFF) << 24) |
        ((data[11] as u64 & 0xFF) << 32) |
        ((data[10] as u64 & 0xFF) << 40) |
        ((data[9] as u64 & 0xFF) << 48) |
        ((data[8] as u64 & 0xFF) << 56) 
        // ((data[7] as u127 & 0xFF) << 64) |
        // ((data[6] as u127 & 0xFF) << 72) |
        // ((data[5] as u127 & 0xFF) << 80) |
        // ((data[4] as u127 & 0xFF) << 88) |
        // ((data[3] as u127 & 0xFF) << 96) |
        // ((data[2] as u127 & 0xFF) << 104) |
        // ((data[1] as u127 & 0xFF) << 112) |
        // ((data[0] as u127 & 0xFF) << 120) 
    }

        fn bytesToU641(data:[u8;16]) -> u64 {

        ((data[7] as u64 & 0xFF)) |
        ((data[6] as u64 & 0xFF) << 8) |
        ((data[5] as u64 & 0xFF) << 16) |
        ((data[4] as u64 & 0xFF) << 24) |
        ((data[3] as u64 & 0xFF) << 32) |
        ((data[2] as u64 & 0xFF) << 40) |
        ((data[1] as u64 & 0xFF) << 48) |
        ((data[0] as u64 & 0xFF) << 56) 
    }

    fn u64_to_bytes(val1: u64,val2:u64) -> [u8; 16] {
        let mut bytes = [0; 16];
    
        for i in 0..8 {
            bytes[15 - i] = ((val2 >> (i * 8)) & 0xFF) as u8;
        }
        for i in 8..16 {
            bytes[15 - i] = ((val1 >> ((i-8) * 8)) & 0xFF) as u8;
        }

        // for i in 0..16 {
        //     if i < 8 {
        //         bytes[15 - i] = ((val1 >> (i * 8)) & 0xFF) as u8;
        //     } else {
        //         bytes[15 - i] = ((val2 >> ((i-8) * 8)) & 0xFF) as u8;
        //     }
        // }
        bytes
    }


    pub fn encode(data:u128, encodings: [[u8;16];8])->[[u8;16];8] {

        let mut labels:[[u8;16];8] = [[0;16];8];
        let delta1 = 0x0db998f9a99ba256;
        let delta2 = 0x85f440937cdbaf38;


        let bits = u128_to_bits_le(data);

        let mut val1: u64 = 0;
        let mut val2: u64 = 0;

        for i in 0..labels.len() {
            let label1 = ChaChaEncoder::bytesToU641(encodings[i]);
            let label2 = ChaChaEncoder::bytesToU642(encodings[i]);
            
            if (bits[i] != 0){
                val1 = label1 ^ delta1;
                val2 = label2 ^ delta2;
            } else {
                val1 = label1;
                val2 = label2;
            }

            

            let encoded = ChaChaEncoder::u64_to_bytes(val1,val2);
            // let mut encoded_label = [0;16];
            // for j in 0..encoded_label.len() {
            //     encoded_label[j] = encoded[j];
            // }

            labels[i] = encoded;

        }
  
        labels
    }



}

// fn test_ChaChaEncoder() {
//     let seed = [
//         84, 45, 46, 95, 190, 46, 45, 78, 2, 133, 107, 94, 149, 242, 32, 50, 169, 48, 105, 50, 74, 218, 90, 230, 15, 3, 171, 200, 145, 250, 173, 71
//     ];

//     let mut encoder = ChaChaEncoder::new(seed);

//     let id = [
//         0, 0, 0, 0,
//         158, 75, 139, 198,
//         129, 80, 196, 151
//     ];

//     let labels = encoder.get_encodings(id);

//     let expected = [
//         [
//             0x50, 0xe0, 0x62, 0xbe, 0xd3, 0x0b, 0xba, 0x62, 0x22, 0xcf, 0x6b, 0x26, 0xbd, 0xd9, 0x45, 0x0a
//         ], [
//             0xe2, 0x92, 0xd0, 0xe8, 0xeb, 0x07, 0xcc, 0x72, 0x8b, 0xea, 0xea, 0x81, 0x24, 0x01, 0xea, 0xe2
//         ], [
//             0x38, 0xfe, 0xf2, 0x75, 0xe9, 0x38, 0xef, 0xf9, 0x7c, 0x2c, 0x00, 0x69, 0x2b, 0x50, 0x68, 0x42
//         ], [
//             0x0b, 0x5d, 0x4e, 0x68, 0x29, 0x14, 0x9f, 0xaa, 0x60, 0x4d, 0x17, 0xde, 0x54, 0x1d, 0x0b, 0xa2
//         ], [
//             0x6f, 0x57, 0xb4, 0x53, 0x01, 0x86, 0x7c, 0x7e, 0x39, 0x13, 0xd9, 0x35, 0x19, 0x0a, 0x66, 0x00
//         ], [
//             0x8b, 0xa3, 0x9c, 0x9e, 0x89, 0xf3, 0x95, 0xcd, 0x15, 0x73, 0x69, 0xb1, 0xb3, 0xb7, 0xbc, 0xd7
//         ], [
//             0xa3, 0x12, 0x5e, 0x9e, 0x8b, 0x2b, 0xf9, 0x04, 0x93, 0xe2, 0xbb, 0xa1, 0x36, 0xbf, 0x1d, 0x8b
//         ], [
//             0x7b, 0x6b, 0x08, 0xf2, 0xd8, 0x9f, 0xf8, 0x02, 0x1d, 0x00, 0xbf, 0xa6, 0x79, 0x70, 0x83, 0xce
//         ]
//     ];

//     for i in 0..expected.len() {
//         for j in 0..expected[i].len() {
//             assert_eq(labels[i][j], expected[i][j]);
//         }
//     }
// }

// #[test]
// fn test_ChaChaEncoder_encode() {
//     let seed = [
//         84, 45, 46, 95, 190, 46, 45, 78, 2, 133, 107, 94, 149, 242, 32, 50, 169, 48, 105, 50, 74, 218, 90, 230, 15, 3, 171, 200, 145, 250, 173, 71
//     ];

//     let mut encoder = ChaChaEncoder::new(seed);

//     let id = [
//         0, 0, 0, 0, 158,
//         75, 139, 198, 129, 80,
//         196, 151
//     ];

//     let labels = encoder.get_encodings(id);

//     let data = 125;
//     let encoded_labels = ChaChaEncoder::encode(data, labels);
//     let expected = [
//         [
//             93, 89, 250, 71, 122, 144,
//             24, 52, 167, 59, 43, 181,
//             193, 2, 234, 50
//         ],
//         [
//             226, 146, 208, 232, 235,
//             7, 204, 114, 139, 234,
//             234, 129, 36, 1, 234,
//             226
//         ],
//         [
//             53, 71, 106, 140, 64,
//             163, 77, 175, 249, 216,
//             64, 250, 87, 139, 199,
//             122
//         ],
//         [
//             6, 228, 214, 145, 128,
//             143, 61, 252, 229, 185,
//             87, 77, 40, 198, 164,
//             154
//         ],
//         [
//             98, 238, 44, 170, 168,
//             29, 222, 40, 188, 231,
//             153, 166, 101, 209, 201,
//             56
//         ],
//         [
//             134, 26, 4, 103, 32, 104,
//             55, 155, 144, 135, 41, 34,
//             207, 108, 19, 239
//         ],
//         [
//             174, 171, 198, 103, 34,
//             176, 91, 82, 22, 22,
//             251, 50, 74, 100, 178,
//             179
//         ],
//         [
//             123, 107, 8, 242, 216,
//             159, 248, 2, 29, 0,
//             191, 166, 121, 112, 131,
//             206
//         ]
//     ];

//     for i in 0..expected.len() {
//         std::println("start");
//         std::println(encoded_labels[i]);
//         std::println(expected[i]);
//         std::println("end");
//         for j in 0..expected[i].len() {
//             assert_eq(encoded_labels[i][j], expected[i][j]);
//         }
//     }
// }
