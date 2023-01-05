use miden::{Assembler, Program, ProgramInputs, ProgramOutputs, ProofOptions};
use miden_core::{chiplets, Felt, FieldElement, StarkField};
use rustbench::Benchmark;

pub struct Job {
    merkle_path: Vec<chiplets::hasher::Digest>,
    num_iter: u32,
    program: Program,
    program_input: ProgramInputs,
    proof_options: ProofOptions,
    program_outputs: ProgramOutputs,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![10, 100, 1000]
}

impl Benchmark for Job {
    const NAME: &'static str = "merkle_rescue_prime";
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
            # compute merkle root
                repeat.{}
                    rphash
                end
            end",
            num_iter + 1
        );

        // Compiling the program
        let assembler = Assembler::default();
        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");

        // Default (96 bits of security)
        let proof_options = ProofOptions::with_96_bit_security();

        let program_outputs = ProgramOutputs::new(vec![], vec![]);

        // We first create a vector [0, 1, 2, ..., num_iter] as basis of our Merkle path
        let merkle_path_base_vector: Vec<u64> = (0..num_iter as u64 + 2).collect();

        // Then we use the vector to create a Merkle path - a list of hashes - and 
        // hash every element. [[Felt(0), [Felt(1), ..., Felt(num_iter)]
        let merkle_path = merkle_path_base_vector
            .iter()
            .map(|x| chiplets::hasher::hash_elements(&[Felt::new(*x)]))
            .collect::<Vec<_>>();

        // Per job size we now create ProgramInputs of the required lenght.
        // Per Merkle leaf and Rescue Prime hash we have four u64 values as input
        let merkle_path_as_u64 = merkle_path
            .iter()
            .map(|x| x.as_elements())
            .map(|x| x.iter().map(|y| y.as_int()).collect::<Vec<u64>>())
            .collect::<Vec<_>>();

        let mut input = Vec::<u64>::new();

        for hash in merkle_path_as_u64 {
            for element in hash {
                input.push(element)
            }
        }

        // Now as a last step, we need to add two non-hashed elements to the stack
        // The element we want to prove inclusion (num_iter + 2) for
        // and its neighbour (num_iter + 1).
        //input.push(num_iter as u64 + 1);
        //input.push(num_iter  as u64 + 2);

        let program_input = ProgramInputs::from_stack_inputs(&input).unwrap();

        Job {
            merkle_path,
            num_iter,
            program,
            program_input,
            proof_options,
            program_outputs,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.num_iter
    }

    /// Compute on host CPU
    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        // We want to use the Merkle path
        let merkle_path = &mut self.merkle_path.to_owned();

        // We need to hash first the neighbour and the number we want to proof the inclusion for
        //let neighbour = [Felt::new(self.num_iter as u64 + 1)];
        //let neighbour_hashed = chiplets::hasher::hash_elements(&neighbour);

        //let number_to_be_proven = [Felt::new(self.num_iter as u64 + 2)];
        //let number_to_be_proven_hashed = chiplets::hasher::hash_elements(&number_to_be_proven);

        let mut merge_digest = [merkle_path.pop().unwrap(), merkle_path.pop().unwrap()];
        merge_digest.reverse();

        let mut output = [Felt::ZERO; 4];

        for _ in 0..self.num_iter + 1 {
            let output_digest = chiplets::hasher::merge(&merge_digest);

            if !merkle_path.is_empty() {
                merge_digest = [merkle_path.pop().unwrap(), output_digest];
            }
            output = output_digest.as_elements().try_into().unwrap();
        }

        let result = output.iter().map(|x| x.as_int()).collect::<Vec<u64>>();
        Some(result)
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

        self.program_outputs = output;

        (stack, proof)
    }

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let program = &self.program;
        let program_outputs = &self.program_outputs;
        let program_input_u64 = self
            .program_input
            .stack_init()
            .iter()
            .map(|x| x.as_int())
            .rev()
            .collect::<Vec<u64>>();

        let stark_proof = proof.clone();

        let result = miden::verify(
            program.hash(),
            &program_input_u64,
            program_outputs,
            stark_proof,
        )
        .map_err(|err| format!("Program failed verification! - {}", err));

        match result {
            Ok(_) => true,
            Err(_err) => false,
        }
    }
}
