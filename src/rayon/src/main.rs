use std::time::{Duration, Instant};

use rayon::prelude::*;

use plot::plot_results;

mod plot;

fn main() {
    let size = 10 * 1024 * 1024;
    let data: Vec<_> = (0..size).collect();
    let ts: Vec<_> = (1..=1000)
        .step_by(10)
        .map(|c| (c, compare(&data, c)))
        .collect();
    // for (chunk, (t1, t2)) in ts {
    //     println!("chunk={chunk:4}: seq={t1:?}, par={t2:?}");
    // }
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
