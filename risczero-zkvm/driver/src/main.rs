mod benches;

use rand::{rngs::StdRng, RngCore, SeedableRng};
use risc0_zkvm::host::{Prover, Receipt};
use std::time::{Duration, Instant};
use thiserror::Error;

pub struct Metrics {
    job_name: String,
    job_size: u32,
    host_duration: Duration,
    proof_duration: Duration,
    verify_duration: Duration,
    falsify_duration: Duration,
    journal_bytes: u32,
    seal_bytes: u32,
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
            journal_bytes: 0,
            seal_bytes: 0,
        }
    }
}

#[derive(Error, Debug)]
pub enum BenchmarkError<E> {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("ZKVM host exception")]
    ZkvmHostException(#[from] risc0_zkvm::host::Exception),
    #[error("Could not falsify altered journal")]
    CouldNotFalsifyAlteredJournal,
    #[error("Job error {0:?}")]
    JobError(E),
}

pub trait Benchmark
where
    Self: Sized,
{
    type OutType;
    type Error: core::fmt::Debug;
    const METHOD_ID: &'static [u8];
    const METHOD_PATH: &'static str;

    fn new_jobs(rand: &mut StdRng) -> Vec<Self>;

    fn name(&self) -> String;

    fn job_size(&self) -> u32;

    fn host_compute(&self) -> Result<Self::OutType, BenchmarkError<Self::Error>>;

    fn new_prover(&self) -> Result<Prover, BenchmarkError<Self::Error>>;

    fn check_journal(
        &self,
        host_compute: Self::OutType,
        journal: Vec<u32>,
    ) -> Result<(), BenchmarkError<Self::Error>>;

    fn run(&self, rand: &mut StdRng) -> Result<Metrics, BenchmarkError<Self::Error>> {
        let mut metrics = Metrics::new(self.name(), self.job_size());

        // Perform the host-side variant of the computation
        let host_compute = {
            let start = Instant::now();
            let result = self.host_compute()?;
            metrics.host_duration = start.elapsed();

            result
        };
        println!("Host-side compute complete");

        // Create prover object
        let prover = self.new_prover()?;
        println!("Prover created and initialized");

        // Generate the proof
        let receipt: Receipt = {
            let start = Instant::now();
            let result = prover.run()?;
            metrics.proof_duration = start.elapsed();

            metrics.journal_bytes = (result.get_journal()?.len()) as u32;
            metrics.seal_bytes = (result.get_seal()?.len() * 4) as u32;
            result
        };
        println!("Proof generated");

        // Parse and check the journal
        {
            self.check_journal(host_compute, receipt.get_journal_vec()?)?;
        }
        println!("Journal contents parsed and verified");

        // Verify the proof
        {
            let start = Instant::now();
            receipt.verify(Self::METHOD_ID)?;
            metrics.verify_duration = start.elapsed();
        }
        println!("Proof verified");

        // Verify that the journal cannot be altered
        {
            let journal = {
                let mut journal = Vec::from(receipt.get_journal()?);
                let alter_i = rand.next_u32() as usize % journal.len();

                let bit = journal[alter_i] ^ 1;
                let rest = journal[alter_i] & 0xfe;
                journal[alter_i] = rest | bit;

                journal
            };
            let modified_receipt = Receipt::new(&journal, receipt.get_seal()?)?;

            let start = Instant::now();
            let result = modified_receipt.verify(Self::METHOD_ID);
            metrics.falsify_duration = start.elapsed();
            match result {
                Ok(_) => return Err(BenchmarkError::CouldNotFalsifyAlteredJournal),
                Err(e) => println!(
                    "Altered receipt falsified (this is a good thing): {:?}",
                    e.what()
                ),
            }
        }

        Ok(metrics)
    }
}

fn run_jobs<B: Benchmark>(rand: &mut StdRng, all_jobs: Vec<B>) -> Vec<Metrics> {
    let mut all_metrics: Vec<Metrics> = Vec::new();
    let mut error_count: u32 = 0;

    for job in all_jobs {
        let job_number = all_metrics.len();

        println!("");
        println!("+ begin job_number:   {}", job_number);

        let job_metrics = match job.run(rand) {
            Ok(job_metrics) => job_metrics,
            Err(job_error) => {
                println!("+ error job_number:   {} {:?}", job_number, job_error);
                error_count = error_count + 1;
                break;
            }
        };

        println!("+ job_name:           {:?}", &job_metrics.job_name);
        println!("+ job_size:           {:?}", &job_metrics.job_size);
        println!("+ host_duration:      {:?}", &job_metrics.host_duration);
        println!("+ proof_duration:     {:?}", &job_metrics.proof_duration);
        println!("+ verify_duration:    {:?}", &job_metrics.verify_duration);
        println!("+ falsify_duration:   {:?}", &job_metrics.falsify_duration);
        println!("+ journal_bytes:      {:?}", &job_metrics.journal_bytes);
        println!("+ seal_bytes:         {:?}", &job_metrics.seal_bytes);
        println!("+ end job_number:     {}", job_number);

        all_metrics.push(job_metrics);
    }

    println!("- jobs:               {}", all_metrics.len());
    println!("- errors:             {}", error_count);
    println!("- done");

    all_metrics
}

fn main() {
    let mut rand = StdRng::seed_from_u64(1337);

    let _big_sha2 = {
        let jobs = benches::big_sha2::Job::new_jobs(&mut rand);
        run_jobs(&mut rand, jobs)
    };
}
