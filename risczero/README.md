# RISC Zero

Benchmarks for [RISC Zero](https://github.com/risc0/risc0).

## Benchmarks

### `big_sha2`

Computes the SHA2-256 hash of large random buffers of various sizes.

## Running the benchmarks

```console
$ RUST_LOG=info cargo run --release -- --out metrics.csv all
```
