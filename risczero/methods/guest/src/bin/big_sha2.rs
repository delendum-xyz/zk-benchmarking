#![no_std]
#![no_main]

use risc0_zkvm::guest::{env, sha};
use risc0_zkvm::sha::Sha;

risc0_zkvm::entry!(main);

pub fn main() {
    let hasher = sha::Impl {};
    let data: &[u8] = env::send_recv(0, &[]);

    let hash = hasher.hash_bytes(data);
    env::commit(hash)
}
