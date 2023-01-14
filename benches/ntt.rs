use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    divisors::{divisors, is_smooth, split},
    ntt,
    ntt_old::Fft, permute,
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    Naive,
    Old,
    Ntt,
    Inverse,
    Small,
    Transpose,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ignored, but required by cargo.
    #[arg(long)]
    bench: bool,

    /// Algorithm to test
    #[arg(value_enum, default_value = "ntt")]
    algo: Algorithm,

    /// Test on all supported numbers instead of just the smooth ones.
    #[arg(long)]
    all: bool,

    /// Log₂ of the maximum number of values to test
    #[arg(default_value = "20")]
    max_exponent: usize,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();
    let cli = Args::parse();

    let max_exp = cli.max_exponent;
    let max_size = 1 << max_exp;
    eprintln!("Generating random input of size 2^{max_exp} = {max_size}");
    let mut input = rand_vec(max_size);

    println!("size,duration,throughput");
    for size in divisors()
        .iter()
        .map(|&n| n as usize)
        .filter(|&n| n > 1 && n <= max_size && (cli.all || is_smooth(n)))
    {
        let input = &mut input[..size];

        // Compute matrix dimensions for some algorithms
        let a = split(size);
        let b = size / a;

        // Skip unsupported sizes
        let supported = match cli.algo {
            Algorithm::Small => ntt::small::ntt(input),
            Algorithm::Old => input.len().is_power_of_two(),
            _ => true,
        };
        if !supported {
            continue;
        }

        // Benchmark
        let duration = time(|| match cli.algo {
            Algorithm::Naive => ntt::naive::ntt(input),
            Algorithm::Old => input.fft(),
            Algorithm::Ntt => ntt::ntt(input),
            Algorithm::Inverse => ntt::intt(input),
            Algorithm::Small => drop(ntt::small::ntt(input)),
            Algorithm::Transpose => permute::transpose(input, (a, b)),
        });
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
