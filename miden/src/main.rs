use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod benches;

use benches::iter_blake3;
use benches::iter_rescue_prime;
use benches::iter_sha2;
use benches::merkle_path_rescue_prime;
use rustbench::{init_logging, run_jobs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // CSV output file
    #[arg(long, value_name = "FILE")]
    out: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Eq, PartialEq, Subcommand)]
enum Command {
    All,
    IterBlake3,
    IterSha2,
    IterRescuePrime,
    MerklePathRescuePrime,
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    let prover = String::from("miden");

    if cli.command == Command::All || cli.command == Command::IterBlake3 {
        run_jobs::<iter_blake3::Job>(&prover, &cli.out, iter_blake3::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterSha2 {
        run_jobs::<iter_sha2::Job>(&prover, &cli.out, iter_sha2::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterRescuePrime {
        run_jobs::<iter_rescue_prime::Job>(&prover, &cli.out, iter_rescue_prime::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::MerklePathRescuePrime {
        run_jobs::<merkle_path_rescue_prime::Job>(
            &prover,
            &cli.out,
            merkle_path_rescue_prime::new_jobs(),
        );
    }
}
