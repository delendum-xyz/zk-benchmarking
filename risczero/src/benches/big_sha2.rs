use rand::{rngs::StdRng, RngCore, SeedableRng};
use risc0_zkp::core::{
    sha::{Digest, Sha},
    sha_cpu,
};
use risc0_zkvm::host::{Prover, Receipt};
use risc0_zkvm::serde::to_vec;

use rustbench::Benchmark;

pub struct Job {
    pub guest_input: Vec<u32>,
    pub prover: Option<Prover<'static>>,
}

pub fn new_jobs() -> Vec<Job> {
    let mut rand = StdRng::seed_from_u64(1337);
    let mut jobs = Vec::new();
    for job_size in [1024, 2048, 4096, 8192] {
        let mut guest_input = vec![0; job_size];
        for i in 0..guest_input.len() {
            guest_input[i] = rand.next_u32();
        }

        jobs.push(Job::new(guest_input));
    }
    jobs
}

const METHOD_ID: &'static [u8] = risczero_benchmark_methods::BIG_SHA2_ID;
const METHOD_PATH: &'static str = risczero_benchmark_methods::BIG_SHA2_PATH;

impl Benchmark for Job {
    const NAME: &'static str = "big_sha2";
    type Spec = Vec<u32>;
    type ComputeOut = Digest;
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        (spec.len() * 4) as u32
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        (proof.get_journal().expect("journal").len()) as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        (proof.get_seal().expect("seal").len() * 4) as u32
    }

    fn new(spec: Self::Spec) -> Self {
        Job {
            guest_input: spec,
            prover: None,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.guest_input
    }

    fn host_compute(&mut self) -> Self::ComputeOut {
        let hasher = sha_cpu::Impl {};
        *hasher.hash_words(&self.guest_input)
    }

    fn init_guest_compute(&mut self) {
        let image = std::fs::read(METHOD_PATH).expect("image");
        let mut prover = Prover::new(&image, METHOD_ID).expect("prover");
        prover.add_input(to_vec(&self.guest_input).unwrap().as_slice()).expect("prover input");

        self.prover = Some(prover);
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let prover = self.prover.as_ref().expect("prover");
        let receipt = prover.run().expect("receipt");

        let journal = receipt.get_journal_vec().expect("journal");
        let guest_output: Digest = Digest::from_slice(&journal);
        (guest_output, receipt)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        proof.verify(METHOD_ID).is_ok()
    }

    fn corrupt_proof(&self, proof: Self::ProofType) -> Self::ProofType {
        let journal = {
            let mut journal = Vec::from(proof.get_journal().expect("journal"));
            let alter_i = 3;
            let bit = (journal[alter_i] ^ 1) & 0x01;
            let rest = journal[alter_i] & 0xfe;
            journal[alter_i] = rest | bit;

            journal
        };
        Receipt::new(&journal, proof.get_seal().expect("seal")).expect("receipt")
    }
}
