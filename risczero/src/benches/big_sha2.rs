use rand::{rngs::StdRng, RngCore};
use risc0_zkp::core::{
    sha::{Digest, Sha},
    sha_cpu,
};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::to_vec;
use thiserror::Error;

use crate::{Benchmark, BenchmarkError};

pub struct Job {
    pub guest_input: Vec<u32>,
}

#[derive(Error, Debug)]
pub enum JobError {
    #[error("Hashes do not match! host: `{host_compute}`; guest: `{guest_compute}`")]
    HostMismatch {
        host_compute: Digest,
        guest_compute: Digest,
    },
}

impl Benchmark for Job {
    type OutType = Digest;
    type Error = JobError;
    const METHOD_ID: &'static [u8] = risczero_benchmark_methods::BIG_SHA2_ID;
    const METHOD_PATH: &'static str = risczero_benchmark_methods::BIG_SHA2_PATH;

    fn new_jobs(rand: &mut StdRng) -> Vec<Self> {
        let mut jobs = Vec::new();
        for job_size in [1024, 2048, 4096, 8192] {
            let mut guest_input = vec![0; job_size];
            for i in 0..guest_input.len() {
                guest_input[i] = rand.next_u32();
            }

            jobs.push(Job { guest_input });
        }
        jobs
    }

    fn name(&self) -> String {
        String::from("big_sha2")
    }

    fn job_size(&self) -> u32 {
        self.guest_input.len() as u32
    }

    fn host_compute(&self) -> Result<Self::OutType, BenchmarkError<Self::Error>> {
        let hasher = sha_cpu::Impl {};
        Ok(*hasher.hash_words(&self.guest_input))
    }

    fn new_prover(&self) -> Result<Prover, BenchmarkError<Self::Error>> {
        let method_code = std::fs::read(Self::METHOD_PATH)?;
        let mut prover = Prover::new(&method_code, Self::METHOD_ID)?;
        prover.add_input(to_vec(&self.guest_input).unwrap().as_slice())?;
        Ok(prover)
    }

    fn check_journal(
        &self,
        host_compute: Self::OutType,
        journal: Vec<u32>,
    ) -> Result<(), BenchmarkError<Self::Error>> {
        let guest_compute: Digest = Digest::from_slice(&journal);

        if &host_compute != &guest_compute {
            return Err(BenchmarkError::JobError(JobError::HostMismatch {
                host_compute,
                guest_compute,
            }));
        }

        Ok(())
    }
}
