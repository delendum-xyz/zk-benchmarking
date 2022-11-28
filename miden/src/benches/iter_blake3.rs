use miden::{Assembler, FieldExtension, HashFunction, Program, ProgramInputs, ProofOptions};
use miden_stdlib::StdLibrary;
use rustbench::Benchmark;

pub struct Job {
    num_iter: u32,
    program: Program,
    program_input: ProgramInputs,
    proof_options: ProofOptions,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![100]
}

impl Benchmark for Job {
    const NAME: &'static str = "iter_blake3";
    type Spec = u32;
    type ComputeOut = Vec<u64>;
    type ProofType = miden_prover::StarkProof;

    fn job_size(spec: &Self::Spec) -> u32 {
        *spec
    }

    fn output_size_bytes(output: &Self::ComputeOut, _proof: &Self::ProofType) -> u32 {
        (output.len() * 8) as u32
    }

    fn proof_size_bytes(proof: &Self::ProofType) -> u32 {
        proof.to_bytes().len() as u32
    }

    fn new(num_iter: Self::Spec) -> Self {
        let source = format!(
            "  
            use.std::crypto::hashes::blake3

            begin
                repeat.{}
                    exec.blake3::hash
                end
            end",
            num_iter
        );

        // We need to add here what we want to hash. Atm we hash '1'
        // We can also use secret inputs using the advice provider
        let mut input = Vec::new();
        input.push(1);

        // compiling the program
        let assembler = Assembler::new().with_module_provider(StdLibrary::default());
        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");

        let program_input = ProgramInputs::from_stack_inputs(&input).unwrap();

        // default (96 bits of security)
        let proof_options = ProofOptions::new(
            27,
            8,
            16,
            HashFunction::Blake3_192,
            FieldExtension::Quadratic,
            8,
            256,
        );

        Job {
            num_iter,
            program,
            program_input,
            proof_options,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.num_iter
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = &self.program;
        let program_input = &self.program_input;
        let proof_options = &self.proof_options;

        let (output, proof) = miden::prove(program, program_input, proof_options).expect("results");
        (Vec::from(output.stack()), proof)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, _proof: &Self::ProofType) -> bool {
        true // TODO!
    }

    fn corrupt_proof(&self, proof: Self::ProofType) -> Self::ProofType {
        proof // TODO!
    }
}
