#![no_main]

use risc0_zkvm::guest::env;
use sha2::{Digest as _, Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: Vec<u8> = env::read();

    let mut num_iter: u32;
    num_iter = data[0] as u32;
    num_iter = num_iter | ((data[1] as u32) << 8);
    num_iter = num_iter | ((data[2] as u32) << 16);
    num_iter = num_iter | ((data[3] as u32) << 24);

    // let mut hash = &data[4..];
    let mut hash = Sha256::digest(&data[4..]);
    for _ in 0..num_iter-1 {
        let input = hash.as_slice();
        hash = Sha256::digest(input);
    }
    env::commit(&hash.as_slice())
}
