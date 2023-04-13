use miden::{
    AdviceInputs, Assembler, Kernel, MemAdviceProvider, Program, ProgramInfo, ProofOptions,
    StackInputs, StackOutputs,
};
use miden_crypto::hash::blake::Blake3_256;
use miden_stdlib::StdLibrary;
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
    vec![10, 100]
}

impl Benchmark for Job {
    const NAME: &'static str = "iter_blake3";
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
        let program_inputs = StackInputs::try_from_values(input)
            .map_err(|e| e.to_string())
            .unwrap();

        // compiling the program
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");

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

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = self.program.clone();
        let program_input = self.program_inputs.clone();
        let proof_options = self.proof_options.clone();

        // Creating an empty advice provider
        let advice_inputs = AdviceInputs::default()
            .with_stack_values(vec![])
            .map_err(|e| e.to_string());
        let advice_provider = MemAdviceProvider::from(advice_inputs.unwrap());

        let (output, proof) =
            miden::prove(&program, program_input, advice_provider, proof_options).expect("results");

        let stack_output = output.stack_truncated(8).to_vec();
        self.program_outputs = output;

        (stack_output, proof)
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
