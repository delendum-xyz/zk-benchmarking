use miden::{Assembler, Program, ProgramInputs, ProofOptions};
use miden_core::{FieldElement, Felt, ProgramOutputs, StarkField, chiplets};
use rustbench::Benchmark;

pub struct Job {
    num_iter: u32,
    program: Program,
    program_input: ProgramInputs,
    proof_options: ProofOptions,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![1, 10, 100, 1000]
}

impl Benchmark for Job {
    const NAME: &'static str = "iter_rescue_prime";
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
            # stack start: [a3=0, a2=0, a1=0, a0=0, ...]
            begin
                push.4.0.0.0
                mem_storew.0 # save [4, 0, 0, 0] into memory location 0
                swapw
                push.0.0.0.0 # => stack state [0, 0, 0, 0, a3, a2, a1, a0, 0, 0, 0, 4, ...]
            
            # compute hash chain
                repeat.{}
                    rpperm
                    mem_loadw.0 # overwrite the top word with [4, 0, 0, 0]
                    swapw.2
                    mem_loadw.1 # overwrite top word with [0, 0, 0, 0]
                end
            
                # drop everything but the digest from the stack
                dropw
                swapw
                dropw
            end",
            num_iter
        );

        // We can also transform the input_data into 8-byte arrays and
        // then parse each 8-byte array into a u64
        let input = vec![0u64; 4];

        // Compiling the program
        let assembler = Assembler::default();
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

    /// Compute on VM
    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = &self.program;
        let program_input = &self.program_input;
        let proof_options = &self.proof_options;

        let (output, proof) = miden::prove(program, program_input, proof_options).expect("results");

        let stack = output
            .stack_outputs(4)
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<_>>();

        (stack, proof)
    }

    /// Compute on host CPU
    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        // We also hash a vector of four 0's
        let input = vec![Felt::ZERO; 4];
        let mut output: [Felt; 4] = input.try_into().unwrap();

        for _ in 0..self.num_iter {
            output = chiplets::hasher::hash_elements(&output)
                .as_elements()
                .try_into()
                .unwrap();
        }

        Some(output.iter().map(|x| x.as_int()).collect::<Vec<u64>>())
    }

    fn verify_proof(&self, output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let program = &self.program;
        let stack_inputs = self
            .program_input
            .stack_init()
            .iter()
            .map(|x| x.as_int())
            .collect::<Vec<u64>>();

        let mut stack = output
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<_>>();
        
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
            Err(err) => { println!("{}", err); false},
        }
    }
}
