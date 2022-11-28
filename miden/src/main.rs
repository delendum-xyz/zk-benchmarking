use miden::{Assembler, Program, ProgramInputs, ProofOptions, HashFunction, FieldExtension};
use miden_stdlib::StdLibrary;
use std::time::Instant;

fn main() {

    // Miden assembly blake3 100 times
    let source_blake3 = String::from("  
    use.std::crypto::hashes::blake3

    begin
        repeat.100
            exec.blake3::hash
        end
    end");

    // Miden assembly sha256 100 times
    let _source_sha256 = String::from("    
    use.std::crypto::hashes::sha256
    
    begin
        repeat.100
            exec.sha256::hash
        end
    end");

    // We need to add here what we want to hash. Atm we hash '1'
    // We can also use secret inputs using the advice provider
    let mut input = Vec::new();
    input.push(1);
    
    // compiling the program
    let assembler = Assembler::new().with_module_provider(StdLibrary::default());                
    let program: Program = assembler
        .compile(source_blake3.as_str())
        .expect("Could not compile source");
    
    let input = ProgramInputs::from_stack_inputs(&input).unwrap(); 
    
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

    // execute and prove the program
    let now = Instant::now();
    let (_outputs, _proof) = miden::prove(&program, &input, &proof_options).unwrap();
    println!("done in {} ms", now.elapsed().as_millis())
    
}
