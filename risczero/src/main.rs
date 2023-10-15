use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod benches;

use benches::*;
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
    BigBlake3,
    BigSha2,
    IterBlake3,
    IterSha2,
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    let prover = String::from("risczero");

    if cli.command == Command::All || cli.command == Command::BigSha2 {
        run_jobs::<big_sha2::Job>(&prover, &cli.out, big_sha2::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::BigBlake3 {
        run_jobs::<big_blake3::Job>(&prover, &cli.out, big_blake3::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterSha2 {
        run_jobs::<iter_sha2::Job>(&prover, &cli.out, iter_sha2::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterBlake3 {
        run_jobs::<iter_blake3::Job>(&prover, &cli.out, iter_blake3::new_jobs());
    }
}
