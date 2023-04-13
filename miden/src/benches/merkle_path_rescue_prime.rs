use miden::{
    crypto::MerkleStore,
    math::{Felt, FieldElement},
    AdviceInputs, Assembler, Kernel, MemAdviceProvider, Program, ProgramInfo, ProofOptions,
    StackInputs, StackOutputs, Word,
};
use miden_core::StarkField;
use rustbench::Benchmark;

/// Create a Merkle path of depth 32 and then  
/// the job_size is the number of Merkle paths we verify.
/// So, for job_size=10 we verify 10 Merkle paths of depth 32.
/// Unfortunately in Miden v0.5 I can only create a Sparse Merkle Tree of depth 64.
/// ToDo: Update benchmark when we release Miden v0.6

pub struct Job {
    num_iter: u32,
    program: Program,
    program_info: ProgramInfo,
    program_inputs: StackInputs,
    advice_provider: MemAdviceProvider,
    proof_options: ProofOptions,
    program_outputs: StackOutputs,
    root_as_u64: Vec<u64>,
}

pub fn new_jobs() -> Vec<<Job as Benchmark>::Spec> {
    vec![10, 100, 1000]
}

impl Benchmark for Job {
    const NAME: &'static str = "merkle_rescue_prime";
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
            begin
            # verify a merkle path
                repeat.{}
      
                    # stack = [d, i, R, ...]
                    # duplicate the index and use as counter
                    dup.1 push.1 add movdn.6 
                    # stack = [d, i, R, (i+1) ...]
        
                    # verify merkle path for node i
                    mtree_get
                    # [V, R, (i+1) ...]

                    # now we check if V is what we expect.
                    # our leafs look like this: [0, 0, 0, counter]
                    # get the word with the counter copied back
                    dup.8 push.1 sub 
                    #[i, V, R, (i+1),  ...]
                    
                    # then make it a Word
                    padw drop
                    #[0, 0, 0, i, V, R, (i+1),  ...]


                    # if the two top words are equal, 
                    # [0, 0, 0, i] == V,
                    # we have the expected leaf
                    eqw

                    # fails if top stack element is 0
                    assert 

                    # if it did not fail, we continue with the next iteration
                    dropw dropw
                    #[R, (i+1),  ...]
                    
                    movup.4
                    push.63
                    #[63, (i+1), R,  ...]
                end
                drop drop
            end",
            num_iter
        );

        // Compiling the program
        let assembler = Assembler::default();
        let program = assembler
            .compile(source.as_str())
            .expect("Could not compile source");
        let program_hash = program.hash();
        let kernel = Kernel::default();
        let program_info = ProgramInfo::new(program_hash, kernel);

        // Default (96 bits of security)
        let proof_options = ProofOptions::with_96_bit_security();

        let program_outputs = StackOutputs::new(vec![], vec![]);

        // We first create an array of keys as basis of our Sparse Merkle Tree.
        // For max Job size 1000 we need 1000 keys.
        // We then transform every number into a Word by adding Felt::Zero
        let merkle_leafs_keys: Vec<u64> = (0..1000).collect();
        let mut merkle_leafs: Vec<(u64, Word)> = Vec::new();

        for i in 0..merkle_leafs_keys.len() {
            merkle_leafs.push((
                i as u64,
                [
                    Felt::new(merkle_leafs_keys[i]),
                    Felt::ZERO,
                    Felt::ZERO,
                    Felt::ZERO,
                ],
            ));
        }

        // Now we create a Sparse Merkle Tree with the leafs we just created
        let mut merkle_store = MerkleStore::new();
        let smt_root = merkle_store.add_sparse_merkle_tree(merkle_leafs).unwrap();

        let advice_set = AdviceInputs::default().with_merkle_store(merkle_store);

        let advice_provider = MemAdviceProvider::from(advice_set);

        let root_as_u64 = smt_root.iter().map(|x| x.as_int()).collect::<Vec<u64>>();

        // Per job size we now create as many ProgramInputs as we want to prove.
        // Per Merkle leaf and Rescue Prime hash we have four u64 values as input
        // mtree_get needs the depth of the tree, the index of the leaf and the root of
        // the tree as stack_init: [d, i, R, ...]. If true it returns the leaf V together
        // with the root as output: [V, R, ..]. If false the program fails.
        // So with 100 merkle paths to verfiy we only need to change the index i 100
        // times and store V for each path

        // We want as stack_initi [d, i, R, ...], with d = 2 and i = 0 and R = root_as_u64
        let mut stack_init = Vec::<u64>::new();

        // Depth
        stack_init.push(63);
        // Start index
        stack_init.push(0);
        // Root
        for element in root_as_u64.iter().rev() {
            stack_init.push(*element)
        }
        // We reverse the stack_init to get the correct order
        stack_init.reverse();

        // Finally we create the StackInputs and add the tree as advice_sets
        let program_inputs = StackInputs::try_from_values(stack_init)
            .map_err(|e| e.to_string())
            .unwrap();

        Job {
            num_iter,
            program,
            program_info,
            program_inputs,
            advice_provider,
            proof_options,
            program_outputs,
            root_as_u64,
        }
    }

    fn spec(&self) -> &Self::Spec {
        &self.num_iter
    }

    /// Compute on host CPU
    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        // Actually, there is nothing to host_compute.
        // The assembly program fails, if the merkle path cannot be verified.
        // So we just return the root of the tree.

        Some(self.root_as_u64.clone())
    }

    /// Compute on VM
    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType) {
        let program = self.program.clone();
        let program_inputs = self.program_inputs.clone();
        let proof_options = self.proof_options.clone();
        let advice_provider = self.advice_provider.clone();

        let (output, proof) =
            miden::prove(&program, program_inputs, advice_provider, proof_options)
                .expect("results");

        let mut stack_output = output.stack_truncated(4).to_vec();
        stack_output.reverse();

        self.program_outputs = output;

        (stack_output, proof)
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
            Err(_err) => false,
        }
    }
}
