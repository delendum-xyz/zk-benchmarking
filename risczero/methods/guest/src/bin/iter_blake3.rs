#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::entry!(main);

pub fn main() {
    let input: Vec<u8> = env::read();

    let mut num_iter: u32;
    num_iter = input[0] as u32;
    num_iter = num_iter | ((input[1] as u32) << 8);
    num_iter = num_iter | ((input[2] as u32) << 16);
    num_iter = num_iter | ((input[3] as u32) << 24);

    let mut pre_output = blake3::hash(&input[4..]);
    let mut output = pre_output.as_bytes();
    for _ in 1..num_iter {
        pre_output = blake3::hash(output);
        output = pre_output.as_bytes();
    }

    env::commit(&risc0_zkvm::sha::Digest::try_from(*output).unwrap())
}
