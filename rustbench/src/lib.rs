use std::time::{Duration, Instant};

pub struct Metrics {
    pub job_name: String,
    pub job_size: u32,
    pub proof_duration: Duration,
    pub verify_duration: Duration,
    pub output_bytes: u32,
    pub proof_bytes: u32,
}

impl Metrics {
    pub fn new(job_name: String, job_size: u32) -> Self {
        Metrics {
            job_name,
            job_size,
            proof_duration: Duration::default(),
            verify_duration: Duration::default(),
            output_bytes: 0,
            proof_bytes: 0,
        }
    }

    pub fn println(&self, prefix: &str) {
        println!("{}job_name:           {:?}", prefix, &self.job_name);
        println!("{}job_size:           {:?}", prefix, &self.job_size);
        println!("{}proof_duration:     {:?}", prefix, &self.proof_duration);
        println!("{}verify_duration:    {:?}", prefix, &self.verify_duration);
        println!("{}output_bytes:       {:?}", prefix, &self.output_bytes);
        println!("{}proof_bytes:        {:?}", prefix, &self.proof_bytes);
    }
}

pub trait Benchmark {
    const NAME: &'static str;
    type Spec;
    type ComputeOut: Eq + core::fmt::Debug;
    type ProofType;

    fn job_size(spec: &Self::Spec) -> u32;
    fn output_size_bytes(output: &Self::ComputeOut, proof: &Self::ProofType) -> u32;
    fn proof_size_bytes(proof: &Self::ProofType) -> u32;

    fn new(spec: Self::Spec) -> Self;

    fn spec(&self) -> &Self::Spec;

    fn host_compute(&mut self) -> Option<Self::ComputeOut> {
        None
    }

    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType);
    fn verify_proof(&self, output: &Self::ComputeOut, proof: &Self::ProofType) -> bool;

    fn run(&mut self) -> Metrics {
        let mut metrics = Metrics::new(String::from(Self::NAME), Self::job_size(self.spec()));

        let (g_output, proof) = {
            let start = Instant::now();
            let result = self.guest_compute();
            metrics.proof_duration = start.elapsed();
            result
        };

        if let Some(h_output) = self.host_compute() {
            assert_eq!(g_output, h_output);
        }

        metrics.output_bytes = Self::output_size_bytes(&g_output, &proof);
        metrics.proof_bytes = Self::proof_size_bytes(&proof);

        let _verify_proof = {
            let start = Instant::now();
            let result = self.verify_proof(&g_output, &proof);
            metrics.verify_duration = start.elapsed();
            result
        };

        // assert_eq!(verify_proof, true);
        // assert_eq!(corrupt_verify_proof, false);

        metrics
    }
}

pub fn run_jobs<B: Benchmark>(specs: Vec<B::Spec>) -> Vec<Metrics> {
    let mut all_metrics: Vec<Metrics> = Vec::new();

    for spec in specs {
        let mut job = B::new(spec);
        let job_number = all_metrics.len();

        println!("");

        println!("+ begin job_number:   {} {}", job_number, B::NAME);
        let job_metrics = job.run();
        job_metrics.println("+ ");
        println!("+ end job_number:     {}", job_number);

        all_metrics.push(job_metrics);
    }

    println!("- jobs:               {}", all_metrics.len());
    println!("- done");

    all_metrics
}
