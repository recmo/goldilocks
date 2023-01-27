use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    divisors::{divisors, is_smooth, split},
    ntt, permute,
    utils::gcd,
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    Naive,
    Ntt,
    Inverse,
    Small,
    Transpose,
    Winter,
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

        // Benchmark
        let duration = match cli.algo {
            Algorithm::Naive => time(|| ntt::naive::ntt(input)),
            Algorithm::Ntt => {
                let strat = ntt::strategy(size);
                time(|| strat.ntt(input))
            }
            Algorithm::Inverse => time(|| ntt::intt(input)),
            Algorithm::Small => {
                if !ntt::small::ntt(input) {
                    // Unsupported size
                    continue;
                }
                time(|| ntt::small::ntt(input))
            }
            Algorithm::Transpose => time(|| {
                // Representative of the work in six-step NTT.
                permute::transpose(input, (a, b));
                permute::transpose(input, (b, a));
                permute::transpose(input, (a, b));
            }),
            Algorithm::Winter => {
                if !input.len().is_power_of_two() {
                    // Unsupported size
                    continue;
                }
                let twiddles = winter_math::fft::get_twiddles(input.len());
                let mut input = input
                    .iter()
                    .map(|n| winter_math::fields::f64::BaseElement::new((*n).into()))
                    .collect::<Vec<_>>();
                time(|| winter_math::fft::evaluate_poly(input.as_mut_slice(), &twiddles))
            }
        };
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
