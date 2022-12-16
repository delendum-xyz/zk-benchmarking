use rand::{rngs::StdRng, RngCore, SeedableRng};
use risc0_zkp::core::sha::Digest;
use risc0_zkvm::host::{Prover, Receipt};
use risc0_zkvm::serde::to_vec;
use rustbench::Benchmark;

pub struct Job {
    pub guest_input: Vec<u32>,
    pub prover: Prover<'static>,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    let mut rand = StdRng::seed_from_u64(1337);
    let mut jobs = Vec::new();
    for job_size in [1024, 2048, 4096, 8192] {
        let mut guest_input = vec![0; job_size];
        for i in 0..guest_input.len() {
            guest_input[i] = rand.next_u32();
        }

        jobs.push(guest_input);
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

    fn new(guest_input: Self::Spec) -> Self {
        let image = std::fs::read(METHOD_PATH).expect("image");
        let mut prover = Prover::new(&image, METHOD_ID).expect("prover");
        prover
            .add_input(to_vec(&guest_input).unwrap().as_slice())
            .expect("prover input");

        Job {
            guest_input,
            prover,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.guest_input
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let receipt = self.prover.run().expect("receipt");

        let journal = receipt.get_journal_vec().expect("journal");
        let guest_output: Digest = Digest::from_slice(&journal);
        (guest_output, receipt)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let result = proof.verify(METHOD_ID);

        match result {
            Ok(_) => true,
            Err(err) => { println!("{}", err); false},
        }
    }
}
