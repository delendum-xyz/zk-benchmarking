#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: Vec<u8> = env::read();
    let hash = blake3::hash(&data);
    env::commit(&Digest::try_from(*hash.as_bytes()).unwrap())
}
