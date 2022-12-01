use risc0_zkvm::host::{Prover, Receipt};
use risc0_zkvm::serde::{from_slice, to_vec};
use rustbench::Benchmark;
use sha2::{Digest, Sha256};

pub struct Job {
    pub spec: u32,
    pub prover: Prover<'static>,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![1, 10, 100, 1000, 10000]
}

const METHOD_ID: &'static [u8] = risczero_benchmark_methods::ITER_SHA2_ID;
const METHOD_PATH: &'static str = risczero_benchmark_methods::ITER_SHA2_PATH;

impl Benchmark for Job {
    const NAME: &'static str = "iter_sha2";
    type Spec = u32;
    type ComputeOut = Vec<u8>;
    type ProofType = Receipt;

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec
    }

    fn output_size_bytes(_output: &Self::ComputeOut, proof: &Self::ProofType) -> u32 {
        (proof.get_journal().expect("journal").len()) as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        (proof.get_seal().expect("seal").len() * 4) as u32
    }

    fn new(spec: Self::Spec) -> Self {
        let image = std::fs::read(METHOD_PATH).expect("image");
        let mut prover = Prover::new(&image, METHOD_ID).expect("prover");

        let mut guest_input = Vec::from([0u32; 9]);
        guest_input[0] = spec;
        prover
            .add_input(to_vec(&guest_input).as_ref().expect("guest input"))
            .expect("prover input");

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

        Some(data)
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let receipt = self.prover.run().expect("receipt");

        let journal: Vec<u32> =
            from_slice(&receipt.get_journal_vec().expect("journal")).expect("result");
        let result: Vec<u8> = journal.iter().map(|x| x.to_be_bytes()).flatten().collect();
        (result, receipt)
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
