
#![cfg_attr(feature = "guest", no_std)]
#![no_main]

mod ChaCha20;
mod ChaCha20Rng;
mod ChaChaEncoder;
// use crate::ChaChaEncoder;


#[jolt::provable]
fn chacha(seed: [u8;32], id: [u8;12])->[[u8; 16]; 8] {
    let mut encoder = ChaChaEncoder::ChaChaEncoder::new(seed);
    encoder.get_encodings(id)
}
