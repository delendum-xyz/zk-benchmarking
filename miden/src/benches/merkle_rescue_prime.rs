use miden::{Assembler, Program, ProgramInputs, ProgramOutputs, ProofOptions, Digest, AdviceSet};
use miden_core::{chiplets, Felt, FieldElement, StarkField, Word};
use rustbench::Benchmark;


/// Pick a Merkle path of some depth (e.g., something like 32) and then  
/// the job_size is the number of Merkle paths we verify. 
/// So, for job_size=10 we verify 10 Merkle paths of depth 32, 
/// for job_size=100 we verify 100 Merkle paths of depth 32 etc.

pub struct Job {
    num_iter: u32,
    program: Program,
    program_inputs: Vec<ProgramInputs>,
    proof_options: ProofOptions,
    program_outputs: ProgramOutputs,
    smt: AdviceSet,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![1]
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
            begin
            # verify a merkle path
                mtree_get
            end",
        );

        // Compiling the program
        let assembler = Assembler::default();
        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");

        // Default (96 bits of security)
        let proof_options = ProofOptions::with_96_bit_security();

        let program_outputs = ProgramOutputs::new(vec![], vec![]);

        // We first create an array [0, 1, 2, ..., 1024] as basis of our Sparse Merkle Tree
        // So at index 0 we have 0 expressed in Miden Words, at index 1 we have 1 expressed in Miden words, etc.
        let merkle_leafs_keys: Vec<u64> = (0..4).collect();
        let mut merkle_leafs: Vec<Word> = Vec::new();

        for i in 0..merkle_leafs_keys.len() {
            merkle_leafs.push([Felt::new(merkle_leafs_keys[i]), Felt::ZERO, Felt::ZERO, Felt::ZERO]);
        }

        // Now we create a Sparse Merkle Tree with the leafs we just created
        let smt = AdviceSet::new_sparse_merkle_tree(merkle_leafs_keys, merkle_leafs, 2).unwrap();

        let root_as_u64 = smt.root()
        .iter()
        .map(|x| x.as_int())
        .collect::<Vec<u64>>(); 

        println!("smt_root: {:?}", smt.root());

        // Per job size we now create as many ProgramInputs as we want to prove.
        // Per Merkle leaf and Rescue Prime hash we have four u64 values as input

        let mut program_inputs = Vec::<ProgramInputs>::new();

        for i in 0..num_iter {

            let mut stack_init = Vec::<u64>::new();

            // mtree_get needs the depth of the tree, the index of the leaf and the root of the tree as stack_init
            
            // depth
            stack_init.push(2);

            // index
            stack_init.push(i as u64);
            
            // root of the tree as Word as u64
            for element in root_as_u64.iter().rev() {
                stack_init.push(*element)
                }
            
            // We reverse the input as the stack is LIFO
            stack_init.reverse();
        

            // Finally we create the ProgramInputs and add the tree as advice_sets
            let program_input_i = ProgramInputs::new(&stack_init, &[], vec![smt.clone()]).unwrap();
            program_inputs.push(program_input_i);

            }


        Job {
            num_iter,
            program,
            program_inputs,
            proof_options,
            program_outputs,
            smt,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.num_iter
    }

    /// Compute on host CPU
    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        let mut results: Vec<Vec<u64>> = Vec::new();
        
        // We want to verify as many merkle paths as we have per scenario
        for i in 0..self.num_iter {
            let merkle_path = &mut self.smt.get_path(2, i as u64).unwrap().to_owned();
            
            
            // We need to add the node itself to the merkle path
            let node = [Felt::new(i as u64), Felt::ZERO, Felt::ZERO, Felt::ZERO];
            let mut merge_digest: [Digest; 2] = [node.into(), merkle_path.pop().unwrap().into()];

            let mut output = [Felt::ZERO; 4];

            for _ in 0..31 {
                let output_digest = chiplets::hasher::merge(&merge_digest);
                
                //println!("hashing: {}", j+1);
                output.copy_from_slice(&output_digest.as_elements());
                //println!("Merkle path len: {:?}", merkle_path.len());
                merge_digest = [output.into(), merkle_path.pop().unwrap().into()];
                //println!("Merkle path len after pop: {:?}", merkle_path.len());
                if merkle_path.len() == 0 {
                   // println!("Merkle path: {:?}", merkle_path);
                }
            }
            
            let result = output.iter().map(|x| x.as_int()).collect::<Vec<u64>>();
            results.push(result);
        }
        
        Some(results[0].clone())
    }

    /// Compute on VM
    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = &self.program;
        let program_input = &self.program_inputs[0];
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
            .program_inputs[0]
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
