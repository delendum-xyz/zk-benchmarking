mod benches;

use benches::{iter_blake3, iter_rescue_prime, iter_sha2};
use rustbench::run_jobs;

fn main() {
    //run_jobs::<iter_blake3::Job>(iter_blake3::new_jobs());
    //run_jobs::<iter_sha2::Job>(iter_sha2::new_jobs());
    run_jobs::<iter_rescue_prime::Job>(iter_rescue_prime::new_jobs());
}
