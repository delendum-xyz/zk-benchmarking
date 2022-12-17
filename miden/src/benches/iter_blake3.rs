use miden::{Assembler, Program, ProgramInputs, ProofOptions};
use miden_core::{ProgramOutputs, StarkField};
use miden_crypto::hash::blake::Blake3_256;
use miden_stdlib::StdLibrary;
use rustbench::Benchmark;

pub struct Job {
    num_iter: u32,
    program: Program,
    program_input: ProgramInputs,
    proof_options: ProofOptions,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![1, 10, 100]
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
                    exec.blake3::hash_1to1
                end
            end",
            num_iter
        );

        // We can also transform the input_data into 8-byte arrays and
        // then parse each 8-byte array into a u64
        let input = vec![0u64; 4];

        // compiling the program
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");

        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");

        let program_input = ProgramInputs::from_stack_inputs(&input).unwrap();

        // default (96 bits of security)
        let proof_options = ProofOptions::with_96_bit_security();

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

        let stack = output
            .stack_outputs(8)
            .iter()
            .cloned()
            .map(|x| x)
            .collect::<Vec<_>>();

        (stack, proof)
    }

    fn host_compute(&mut self) -> Option<<Self as Benchmark>::ComputeOut> {
        // We also hash a vector of 32 0's
        let input = [0u8; 32];
        let mut output = input;

        for _ in 0..self.num_iter {
            let pre_output = Blake3_256::hash(&output);

            output = pre_output.into();
        }

        let mut h_output = Vec::<u64>::new();

        for i in 0..=7 {
            let bytes = u32::from_ne_bytes(output[i * 4..(i * 4) + 4].try_into().unwrap());
            h_output.push(bytes.into());
        }

        Some(h_output)
    }

    fn verify_proof(&self, output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let program = &self.program;
        let stack_inputs = self
            .program_input
            .stack_init()
            .iter()
            .map(|x| x.as_int())
            .collect::<Vec<u64>>();

        let mut stack = output.iter().cloned().rev().collect::<Vec<_>>();

        // We need 16 elements in the stack to verify
        let mut stack_rest = vec![0u64; 12];
        stack.append(&mut stack_rest);

        let program_outputs = ProgramOutputs::new(stack, vec![]);

        let stark_proof = proof.clone();

        let result =
            miden_verifier::verify(program.hash(), &stack_inputs, &program_outputs, stark_proof)
                .map_err(|err| format!("Program failed verification! - {}", err));

        match result {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }
}
