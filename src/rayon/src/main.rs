use std::time::{Duration, Instant};

use clap::Parser;
use rayon::prelude::*;

use plot::plot_results;

mod plot;

#[derive(Parser)]
struct Cli {
    /// Size in megabytes.
    #[arg(short, long, default_value_t = 10)]
    size: u64,

    /// A difference between chunk sizes.
    #[arg(short, long, default_value_t = 10)]
    step: usize,

    /// Final vlue of chunk size.
    #[arg(short, long, default_value_t = 1000)]
    max_chunk: usize,
}

fn main() {
    let cli = Cli::parse();

    let size = cli.size * 1024 * 1024;
    let data: Vec<_> = (0..size).collect();
    let ts: Vec<_> = (1..=cli.max_chunk)
        .step_by(cli.step)
        .map(|c| (c, compare(&data, c)))
        .collect();

    println!("{ts:?}");
    plot_results(&ts, "graph.png").expect("save output file");
}

fn compare(data: &[u64], chunk: usize) -> (Duration, Duration) {
    let start = Instant::now();
    let sum1 = data
        .chunks(chunk)
        .map(|c| c.iter().max().unwrap())
        .sum::<u64>();
    let t1 = start.elapsed();

    let start = Instant::now();
    let sum2 = data
        .par_chunks(chunk)
        .map(|c| c.iter().max().unwrap())
        .sum::<u64>();
    let t2 = start.elapsed();

    assert_eq!(sum1, sum2);
    (t1, t2)
}
