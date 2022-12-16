use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod benches;

use benches::iter_rescue_prime;
use benches::iter_blake3;
use benches::iter_sha2;
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
}

fn main() {
    init_logging();
    let cli = Cli::parse();

    if cli.command == Command::All || cli.command == Command::IterBlake3 {
        run_jobs::<iter_blake3::Job>(&cli.out, iter_blake3::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterSha2 {
        run_jobs::<iter_sha2::Job>(&cli.out, iter_sha2::new_jobs());
    }

    if cli.command == Command::All || cli.command == Command::IterRescuePrime {
        run_jobs::<iter_rescue_prime::Job>(&cli.out, iter_rescue_prime::new_jobs());
    }
}
