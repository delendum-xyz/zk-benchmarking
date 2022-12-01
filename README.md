# zk-benchmarking

The goal of this repo is to have a consolidated, neutral, and community-driven place for benchmarking performance of zero knowledge proving systems.

## ZK systems

* [Polygon Miden](https://github.com/maticnetwork/miden)
  * Default security [(96 bits)](https://github.com/maticnetwork/miden/blob/e941cf8dc6397a830d9073c8730389248e82f8e1/air/src/options.rs#L29)
* [RISC Zero](https://github.com/risc0/risc0/)
  * Default security

## Benchmarks

### Iterated hashing

Compute `H(H(H(...H(x))))`, where `H()` is a cryptographic hash function, for some input `x`.

| ZK system     | Hash function |
| ------------- | ------------- |
| Polygon Miden | Blake3        |
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
