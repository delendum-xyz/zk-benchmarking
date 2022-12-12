# ZK Benchmarking

zk-benchmarking is a suite of benchmarks designed to compare different zero-knowledge proof libraries. Zero-knowledge proof libraries are used to enable privacy-preserving transactions and other cryptographic protocols, and it's important to be able to compare the performance of different implementations to choose the best one for a given use case.

With zk-benchmarking, you can run a suite of standardized benchmarks against different zero-knowledge proof libraries and see how they perform in terms of speed, memory usage, and other metrics. This can help you make informed decisions about which library is the best choice for your needs.

Features:

* A collection of standard benchmarks for comparing the performance of different zero-knowledge proof libraries.
* The ability to run the benchmarks against multiple libraries and compare the results.
* Outputs results in a easy-to-read format, including graphs and tables.

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

What are the relevant factors when picking a ZK system?

### Performance

We measure execution time and memory requirements in various standard hardware environments, thereby allowing each proof system to showcase its real-world behavior when running on commonly available systems.

For each benchmark, we measure the performance of these tasks:

* Proof generating
* Verifying a valid proof
* Rejecting an invalid proof

### Security

* What is the security model? 
* How many "bits of security" does the system offer? 
* Is it post-quantum? 
* What hash functions does it support?

### Ease of building new apps

* How hard is it to write new apps for the platform?
* Does it require custom circuits?
* Does it support custom circuits?
* Are there libraries and developer tools? How mature are they?

### Upgradability

* Is the VM tightly coupled to its cryptographic core? Or is there an abstraction layer between them?
* If a new breakthrough in ZKPs took place tomorrow, would the VM be able to incorporate the new advances without breaking existing apps?

## Benchmarks

We start with smaller computations and will eventually move on to larger end-to-end scenarios (e.g., integrity of modified images).

### Iterated hashing

(Scenario type: building block)

Compute `H(H(H(...H(x))))`, where `H()` is a cryptographic hash function, for some input `x`.

| ZK system     | Hash function |
| ------------- | ------------- |
| Polygon Miden | Blake3        |
| Polygon Miden | Rescue        |
| Polygon Miden | SHA2-256      |
| RISC Zero     | SHA2-256      |

### Merkle inclusion

(Scenario type: building block)

*Coming soon!*

### Recursion

*Coming soon!*

## Running the benchmarks

### Requirements

* [`cargo`](https://doc.rust-lang.org/stable/cargo/)

### Running all of the benchmarks

```console
$ ./all.sh
```

## Contributing

If you would like to contribute to zk-benchmarking, please fork the repository and submit a pull request with your changes. All contributions are welcome, including new benchmarks and improvements to existing ones.


