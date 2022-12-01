mod benches;

use benches::*;
use rustbench::run_jobs;

fn main() {
    // run_jobs::<big_sha2::Job>(big_sha2::new_jobs());
    run_jobs::<iter_sha2::Job>(iter_sha2::new_jobs());
}
