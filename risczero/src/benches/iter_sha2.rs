use risc0_zkvm::serde::to_vec;
use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use rustbench::Benchmark;
use sha2::{Digest, Sha256};

pub struct Job<'a> {
    pub spec: u32,
    pub env: ExecutorEnv<'a>,
    pub image: Vec<u8>,
}

pub fn new_jobs() -> Vec<<Job<'static> as Benchmark>::Spec> {
    vec![1, 10, 100]
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::ITER_SHA2_ID;
const METHOD_PATH: &'static str = risczero_benchmark_methods::ITER_SHA2_PATH;

impl Benchmark for Job<'_> {
    const NAME: &'static str = "iter_sha2";
    type Spec = u32;
    type ComputeOut = risc0_zkvm::sha::Digest;
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        (proof.journal.len()) as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        (proof
            .inner
            .flat()
            .unwrap()
            .iter()
            .fold(0, |acc, segment| acc + segment.get_seal_bytes().len())) as u32
    }

    fn new(spec: Self::Spec) -> Self {
        let image = std::fs::read(METHOD_PATH).expect("image");

        let mut guest_input = Vec::from([0u8; 36]);
        guest_input[0] = spec as u8;
        guest_input[1] = (spec >> 8) as u8;
        guest_input[2] = (spec >> 16) as u8;
        guest_input[3] = (spec >> 24) as u8;

        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&guest_input).unwrap())
            .build()
            .unwrap();

        Job { spec, env, image }
    }

    fn spec(&self) -> &Self::Spec {
        &self.spec
    }

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        let mut data = Vec::from([0u8; 32]);

        for _i in 0..self.spec {
            let mut hasher = Sha256::new();
            hasher.update(&data);
            data = hasher.finalize().to_vec();
        }

        Some(risc0_zkvm::sha::Digest::try_from(data.as_slice()).unwrap())
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let receipt = default_prover()
            .prove_elf(self.env.clone(), &self.image)
            .expect("receipt");

        let result = risc0_zkvm::sha::Digest::try_from(receipt.journal.clone())
            .unwrap()
            .try_into()
            .unwrap();
        (result, receipt)
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
