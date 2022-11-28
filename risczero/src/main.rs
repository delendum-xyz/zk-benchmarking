mod benches;

use benches::big_sha2;
use rustbench::run_jobs;

fn main() {
    run_jobs::<big_sha2::Job>(big_sha2::new_jobs());
}
