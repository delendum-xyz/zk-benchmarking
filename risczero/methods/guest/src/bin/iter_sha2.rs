#![no_std]
#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::{
    sha,
    sha::{Digest, Sha256},
};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: &[u8] = env::send_recv(0, &[]);

    let mut num_iter: u32;
    num_iter = data[0] as u32;
    num_iter = num_iter | ((data[1] as u32) << 8);
    num_iter = num_iter | ((data[2] as u32) << 16);
    num_iter = num_iter | ((data[3] as u32) << 24);

    let mut hash = &data[4..];
    for _ in 0..num_iter {
        hash = sha::Impl::hash_bytes(hash).as_bytes();
    }

    env::commit(&Digest::try_from(hash).unwrap())
}
