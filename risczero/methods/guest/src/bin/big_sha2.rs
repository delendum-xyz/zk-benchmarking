#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::{sha, sha::Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: Vec<u8> = env::read();

    let hash = sha::Impl::hash_bytes(&data);
    env::commit(hash)
}
