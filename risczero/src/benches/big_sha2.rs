use rand::{rngs::StdRng, RngCore, SeedableRng};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::sha::{Digest, DIGEST_WORDS};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use rustbench::Benchmark;

pub struct Job<'a> {
    pub guest_input: Vec<u8>,
    pub env: ExecutorEnv<'a>,
    pub image: Vec<u8>,
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    let mut rand = StdRng::seed_from_u64(1337);
    let mut jobs = Vec::new();
    for job_size in [1024, 2048, 4096, 8192] {
        let mut guest_input = vec![0; job_size];
        for i in 0..guest_input.len() {
            guest_input[i] = rand.next_u32() as u8;
        }

        jobs.push(guest_input);
    }
    jobs
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::BIG_SHA2_ID;
const METHOD_PATH: &'static str = risczero_benchmark_methods::BIG_SHA2_PATH;

impl Benchmark for Job<'_> {
    const NAME: &'static str = "big_sha2";
    type Spec = Vec<u8>;
    type ComputeOut = Digest;
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        spec.len() as u32
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        proof.journal.len() as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        (proof
            .inner
            .flat()
            .unwrap()
            .iter()
            .fold(0, |acc, segment| acc + segment.get_seal_bytes().len())) as u32
    }

    fn new(guest_input: Self::Spec) -> Self {
        let image = std::fs::read(METHOD_PATH).expect("image");
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&guest_input).unwrap())
            .build()
            .unwrap();

        Job {
            guest_input,
            env,
            image,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.guest_input
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let receipt = default_prover()
            .prove_elf(self.env.clone(), &self.image)
            .expect("receipt");

        let guest_output: Digest = from_slice::<Vec<u8>, _>(&receipt.journal)
            .unwrap()
            .try_into()
            .unwrap();
        (guest_output, receipt)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let result = proof.verify(METHOD_ID);

        match result {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }
}
