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
 * Default security [(100 bits)](https://github.com/risc0/risc0/#security)
 
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
Iterated hashing is an essetial building block for Merkle tree structures and whenever one needs to succinctly commit larger amounts of data.
 
Compute `H(H(H(...H(x))))`, where `H()` is a cryptographic hash function, for some input `x`. As input `x` we chose a 32-bytes input `[0_u8; 32]` and `job_size` defines the number of iterations.
 
| ZK system     | Hash function |
| ------------- | ------------- |
| Polygon Miden | Blake3        |
| Polygon Miden | Rp64_256      |
| Polygon Miden | SHA2-256      |
| RISC Zero     | SHA2-256      |
 
RISC Zero also benchmarks in `big_sha2` the one-time hashing of large random buffers. Here the `job_size` indicates the size of the buffer.
___
Here are results on a 64 core Graviton 3 and an Apple M2 machine:


<table>
    <thead>
        <tr>
            <th rowspan=2 colspan=2>Proving time in s</th>
            <th colspan=3>SHA256</th>
            <th colspan=3>BLAKE3</th>
            <th colspan=4>RP64_256</th>
        </tr>
        <tr>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1000</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td colspan=12>Apple M2 8GB RAM </td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">Miden</td>
            <td>0.27</td>
            <td>1.91</td>
            <td>40.39</td>
            <td>0.15</td>
            <td>0.96</td>
            <td>9.87</td>
            <td>0.04</td>
            <td>0.03</td>
            <td>0.05</td>
            <td>0.28</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">RISC Zero</td>
            <td>0.63</td>
            <td>1.29</td>
            <td>5.48</td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
        <tr>
            <td colspan=12>AWS Graviton 3</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">Miden</td>
            <td>0.12</td>
            <td>0.49</td>
            <td>3.99</td>
            <td>0.10</td>
            <td>0.33</td>
            <td>2.06</td>
            <td>0.04</td>
            <td>0.04</td>
            <td>0.05</td>
            <td>0.13</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">RISC Zero</td>
            <td>0.18</td>
            <td>0.40</td>
            <td>1.59</td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
    </tbody>
</table>

___

<table>
    <thead>
        <tr>
            <th rowspan=2 colspan=2>Verification time in ms</th>
            <th colspan=3>SHA256</th>
            <th colspan=3>BLAKE3</th>
            <th colspan=4>RP64_256</th>
        </tr>
        <tr>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1000</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td colspan=12>Apple M2 8GB RAM </td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">Miden</td>
            <td>2.26</td>
            <td>2.42</td>
            <td>3.73</td>
            <td>2.68</td>
            <td>2.56</td>
            <td>2.52</td>
            <td>2.44</td>
            <td>2.24</td>
            <td>2.28</td>
            <td>2.42</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">RISC Zero</td>
            <td>1.81</td>
            <td>1.92</td>
            <td>2.44</td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
        <tr>
            <td colspan=12>AWS Graviton 3</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">Miden</td>
            <td>3.05</td>
            <td>3.26</td>
            <td>3.54</td>
            <td>3.27</td>
            <td>3.24</td>
            <td>3.47</td>
            <td>2.79</td>
            <td>2.77</td>
            <td>2.81</td>
            <td>3.04</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">RISC Zero</td>
            <td>2.81</td>
            <td>3.03</td>
            <td>4.05</td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
    </tbody>
</table>

___
<table>
    <thead>
        <tr>
            <th rowspan=2 colspan=2>Proof sizes in bytes</th>
            <th colspan=3>SHA256</th>
            <th colspan=3>BLAKE3</th>
            <th colspan=4>RP64_256</th>
        </tr>
        <tr>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1</th>
            <th>10</th>
            <th>100</th>
            <th>1000</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> </td>
            <td style="text-align:left">Miden</td>
            <td>72356</td>
            <td>89776</td>
            <td>107560</td>
            <td>67699</td>
            <td>83300</td>
            <td>100752</td>
            <td>53203</td>
            <td>53284</td>
            <td>57529</td>
            <td>72742</td>
        </tr>
        <tr>
            <td> </td>
            <td style="text-align:left">RISC Zero</td>
            <td>177684</td>
            <td>187796</td>
            <td>210068</td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
            <td> </td>
        </tr>
       
</table>

____

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
