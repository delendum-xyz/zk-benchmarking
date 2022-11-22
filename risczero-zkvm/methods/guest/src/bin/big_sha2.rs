#![no_main]

use risc0_zkvm_guest::{env, sha};
use risc0_zkp::core::sha::Sha;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let hasher = sha::Impl { };
    let data: Vec<u32> = env::read();

    let hash = hasher.hash_words(&data);
    env::commit(hash)
}
