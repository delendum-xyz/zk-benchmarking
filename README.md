# ZK Benchmarking

The goal of this repo is to have a consolidated, neutral, and community-driven place for benchmarking performance of zero knowledge proving systems.


## ZK systems

* [Polygon Miden](https://github.com/maticnetwork/miden)
  * Default security [(96 bits)](https://github.com/maticnetwork/miden/blob/e941cf8dc6397a830d9073c8730389248e82f8e1/air/src/options.rs#L29)
* [RISC Zero](https://github.com/risc0/risc0/)
  * Default security


## Principles

### Relevant

Our benchmarks measure the performance of realistic, relevant tasks and use cases. This allows third-party engineers to estimate how each proof system would perform in their application.

### Neutral

We do not favor or disfavor any particular proof system. We seek to allow each proof system to show its best features in a variety of realistic test cases.

### Idiomatic

We allow each proof system to interpret each benchmark in whatever way makes the most sense for that system. This allows each proof systems to showcase their performance when being used idiomatically.

### Reproducible

Our measurements are automated and reproducible.


## Factors

### Security

What is the security model? How many "bits of security" does the system offer?

### Performance

We measure execution time and memory requirements in various standard hardware environments, thereby allowing each proof system to showcase its real-world behavior when running on commonly available systems.

For each benchmark, we measure the performance of these tasks:

* Proof generating
* Verifying a valid proof
* Rejecting an invalid proof


## Benchmarks

### Iterated hashing

Compute `H(H(H(...H(x))))`, where `H()` is a cryptographic hash function, for some input `x`.

| ZK system     | Hash function |
| ------------- | ------------- |
| Polygon Miden | Blake3        |
| Polygon Miden | Rescue        |
| Polygon Miden | SHA2-256      |
| RISC Zero     | SHA2-256      |

### Merkle inclusion

*Coming soon!*

## Running the benchmarks

### Requirements

* [`cargo`](https://doc.rust-lang.org/stable/cargo/)

### Running all of the benchmarks

```console
$ ./all.sh
```

## Contributing
Stay tuned
