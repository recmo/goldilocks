use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    ntt::{ntt_naive, recursive::four_step},
    ntt_old::Fft,
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    Naive,
    Old,
    Recursive,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ignored, but required by cargo.
    #[arg(long)]
    bench: bool,

    /// Algorithm to test
    #[arg(value_enum)]
    algo: Algorithm,

    /// Log₂ of the maximum number of values to test
    #[arg(default_value_t = 20)]
    max_exponent: usize,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();
    let cli = Args::parse();

    const MAX_SIZE: usize = 1_usize << 32;

    eprintln!("Generating random input");
    let mut input = rand_vec(MAX_SIZE);

    println!("size,duration,throughput");
    for e in 10..=32 {
        let size = 1 << e;
        if size > input.len() {
            break;
        }
        let input = &mut input[..size];

        let duration = time(|| match cli.algo {
            Algorithm::Naive => ntt_naive(input),
            Algorithm::Old => input.fft(),
            Algorithm::Recursive => four_step(input),
        });
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
