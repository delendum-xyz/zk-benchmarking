#![no_std]
#![no_main]

use risc0_zkp::core::sha::Digest;
use risc0_zkvm::guest::{env, sha};
use risc0_zkvm::sha::Sha;

risc0_zkvm::entry!(main);

pub fn main() {
    let hasher = sha::Impl {};
    let data: &[u8] = env::send_recv(0, &[]);

    let mut num_iter: u32;
    num_iter = data[0] as u32;
    num_iter = num_iter | ((data[1] as u32) << 8);
    num_iter = num_iter | ((data[2] as u32) << 16);
    num_iter = num_iter | ((data[3] as u32) << 24);

    let mut hash = &data[4..];
    for _ in 0..num_iter {
        hash = hasher.hash_bytes(hash).as_bytes();
    }

    env::commit(&Digest::from_bytes(hash))
}
