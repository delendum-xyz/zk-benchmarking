use risc0_zkvm::sha::DIGEST_WORDS;
use risc0_zkvm::{Prover, Receipt};
use rustbench::Benchmark;
use sha2::{Digest, Sha256};

pub struct Job {
    pub spec: u32,
    pub prover: Prover<'static>,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![1, 10, 100]
}

const METHOD_ID: [u32; DIGEST_WORDS] = risczero_benchmark_methods::ITER_SHA2_ID;
const METHOD_PATH: &'static str = risczero_benchmark_methods::ITER_SHA2_PATH;

impl Benchmark for Job {
    const NAME: &'static str = "iter_sha2";
    type Spec = u32;
    type ComputeOut = risc0_zkvm::sha::Digest;
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        (proof.get_journal_bytes().len()) as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        (proof.get_seal_bytes().len()) as u32
    }

    fn new(spec: Self::Spec) -> Self {
        let image = std::fs::read(METHOD_PATH).expect("image");
        let mut prover = Prover::new(&image, METHOD_ID).expect("prover");

        let mut guest_input = Vec::from([0u8; 36]);
        guest_input[0] = spec as u8;
        guest_input[1] = (spec >> 8) as u8;
        guest_input[2] = (spec >> 16) as u8;
        guest_input[3] = (spec >> 24) as u8;
        prover.add_input_u8_slice(guest_input.as_slice());

        Job { spec, prover }
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
        let receipt = self.prover.run().expect("receipt");

        let result = risc0_zkvm::sha::Digest::try_from(receipt.journal.as_slice()).unwrap();
        (result, receipt)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let result = proof.verify(&METHOD_ID);

        match result {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }
}
