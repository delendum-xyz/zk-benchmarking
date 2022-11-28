mod benches;

use rustbench::run_jobs;

fn main() {
    run_jobs(benches::big_sha2::new_jobs());
}
