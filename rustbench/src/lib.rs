use std::time::{Duration, Instant};

pub struct Metrics {
    pub job_name: String,
    pub job_size: u32,
    pub host_duration: Duration,
    pub proof_duration: Duration,
    pub verify_duration: Duration,
    pub falsify_duration: Duration,
    pub output_bytes: u32,
    pub proof_bytes: u32,
}

impl Metrics {
    pub fn new(job_name: String, job_size: u32) -> Self {
        Metrics {
            job_name,
            job_size,
            host_duration: Duration::default(),
            proof_duration: Duration::default(),
            verify_duration: Duration::default(),
            falsify_duration: Duration::default(),
            output_bytes: 0,
            proof_bytes: 0,
        }
    }

    pub fn println(&self, prefix: &str) {
        println!("{}job_name:           {:?}", prefix, &self.job_name);
        println!("{}job_size:           {:?}", prefix, &self.job_size);
        println!("{}host_duration:      {:?}", prefix, &self.host_duration);
        println!("{}proof_duration:     {:?}", prefix, &self.proof_duration);
        println!("{}verify_duration:    {:?}", prefix, &self.verify_duration);
        println!("{}falsify_duration:   {:?}", prefix, &self.falsify_duration);
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

    fn host_compute(&mut self) -> Self::ComputeOut;
    fn init_guest_compute(&mut self);
    fn guest_compute(&mut self) -> (Self::ComputeOut, Self::ProofType);
    fn verify_proof(&self, output: &Self::ComputeOut, proof: &Self::ProofType) -> bool;
    fn corrupt_proof(&self, proof: Self::ProofType) -> Self::ProofType;

    fn run(&mut self) -> Metrics {
        let mut metrics = Metrics::new(String::from(Self::NAME), Self::job_size(self.spec()));

        let h_output = {
            let start = Instant::now();
            let result = self.host_compute();
            metrics.host_duration = start.elapsed();
            result
        };

        self.init_guest_compute();

        let (g_output, proof) = {
            let start = Instant::now();
            let result = self.guest_compute();
            metrics.proof_duration = start.elapsed();
            result
        };

        metrics.output_bytes = Self::output_size_bytes(&g_output, &proof);
        metrics.proof_bytes = Self::proof_size_bytes(&proof);

        let verify_proof = {
            let start = Instant::now();
            let result = self.verify_proof(&g_output, &proof);
            metrics.verify_duration = start.elapsed();
            result
        };

        let corrupt_proof = self.corrupt_proof(proof);

        let corrupt_verify_proof = {
            let start = Instant::now();
            let result = self.verify_proof(&g_output, &corrupt_proof);
            metrics.falsify_duration = start.elapsed();
            result
        };

        assert_eq!(h_output, g_output);
        assert_eq!(verify_proof, true);
        assert_eq!(corrupt_verify_proof, false);

        metrics
    }
}

pub fn run_jobs<B: Benchmark>(all_jobs: Vec<B>) -> Vec<Metrics> {
    let mut all_metrics: Vec<Metrics> = Vec::new();

    for mut job in all_jobs {
        let job_number = all_metrics.len();

        println!("");

        println!("+ begin job_number:   {}", job_number);
        let job_metrics = job.run();
        job_metrics.println("+ ");
        println!("+ end job_number:     {}", job_number);

        all_metrics.push(job_metrics);
    }

    println!("- jobs:               {}", all_metrics.len());
    println!("- done");

    all_metrics
}
