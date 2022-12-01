#![no_main]

use risc0_zkvm_guest::{env, sha};
use risc0_zkp::core::sha::Sha;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let hasher = sha::Impl { };
    let guest_input: Vec<u32> = env::read();

    let num_iter: u32 = guest_input[0];

    let mut hash = &guest_input[1..];
    for _i in 0 .. num_iter {
        hash = hasher.hash_words(&hash).get();
    }

    env::commit(&Vec::from(hash))
}
