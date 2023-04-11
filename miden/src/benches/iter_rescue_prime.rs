use miden::{
    math::{Felt, FieldElement, StarkField},
    AdviceInputs, Assembler, Kernel, MemAdviceProvider, Program, ProgramInfo, ProofOptions,
    StackInputs, StackOutputs,
};
use miden_core::chiplets;
use rustbench::Benchmark;

pub struct Job {
    num_iter: u32,
    program: Program,
    program_info: ProgramInfo,
    program_inputs: StackInputs,
    proof_options: ProofOptions,
    program_outputs: StackOutputs,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![10, 100, 1000]
}

impl Benchmark for Job {
    const NAME: &'static str = "iter_rescue_prime";
    type Spec = u32;
    type ComputeOut = Vec<u64>;
    type ProofType = miden::ExecutionProof;

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
            # compute hash chain
                repeat.{}
                    hash
                end
            end",
            num_iter
        );

        // We can also transform the input_data into 8-byte arrays and
        // then parse each 8-byte array into a u64
        let input = vec![0u64; 4];
        let program_inputs = StackInputs::try_from_values(input)
            .map_err(|e| e.to_string())
            .unwrap();

        // Compiling the program
        let assembler = Assembler::default();
        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");

        let program_hash = program.hash();
        let kernel = Kernel::default();
        let program_info = ProgramInfo::new(program_hash, kernel);

        // default (96 bits of security)
        let proof_options = ProofOptions::with_96_bit_security();

        let program_outputs = StackOutputs::new(vec![], vec![]);

        Job {
            num_iter,
            program,
            program_info,
            program_inputs,
            proof_options,
            program_outputs,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.num_iter
    }

    /// Compute on VM
    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = &self.program;
        let program_input = self.program_inputs.clone();
        let proof_options = self.proof_options.clone();

        // Creating an empty advice provider
        let advice_inputs = AdviceInputs::default()
            .with_stack_values(vec![])
            .map_err(|e| e.to_string());
        let advice_provider = MemAdviceProvider::from(advice_inputs.unwrap());

        let (output, proof) =
            miden::prove(program, program_input, advice_provider, proof_options).expect("results");

        let mut stack_output = output.stack_truncated(4).to_vec();
        stack_output.reverse();

        self.program_outputs = output;

        (stack_output, proof)
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

    fn verify_proof(&self, _output: &Self::ComputeOut, proof: &Self::ProofType) -> bool {
        let program_info = self.program_info.clone();
        let program_inputs = self.program_inputs.clone();
        let program_outputs = self.program_outputs.clone();
        let stark_proof = proof.clone();

        let result = miden::verify(program_info, program_inputs, program_outputs, stark_proof)
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
