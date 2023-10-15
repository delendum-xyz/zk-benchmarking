#![no_std]
#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::{sha, sha::Sha256};

risc0_zkvm::entry!(main);

pub fn main() {
    let data: &[u8] = env::send_recv(0, &[]);

    let hash = sha::Impl::hash_bytes(data);
    env::commit(hash)
}
