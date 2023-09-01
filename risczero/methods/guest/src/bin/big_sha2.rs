#![no_main]

use risc0_zkvm::guest::env;
use sha2::{Digest as _, Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: Vec<u8> = env::read();

    let hash = Sha256::digest(data);
    env::commit(&hash.as_slice())
}
