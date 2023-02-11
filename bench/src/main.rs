use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    divisors::{divisors, is_smooth, split},
    ntt, permute,
    utils::gcd,
};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};
use std::{hint::black_box, io::Write, time::Instant};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    Naive,
    Ntt,
    Inverse,
    Transpose,
    Winter,
    Merkle,
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

fn rand_vec<T>(size: usize) -> Vec<T>
where
    T: Clone + Copy + Default + Send + Sync,
    Standard: Distribution<T>,
{
    let mut result = vec![T::default(); size];
    result
        .par_chunks_mut(1000)
        .for_each_init(thread_rng, |rng, chunk| {
            chunk.iter_mut().for_each(|v| *v = rng.gen());
        });
    result
}

/// Time a function, returning the median of the measurements.
///
/// Unlike Criterion, it will run slow functions only once.
fn time<O>(mut f: impl FnMut() -> O) -> f64 {
    let mut total_duration = 0.0;
    let mut count = 0;
    let mut measurements = Vec::new();
    while total_duration < 1.0 {
        // Do a single run first
        let start = Instant::now();
        let out = black_box(f());
        let end = Instant::now();
        drop(out);
        let run_duration = end.duration_since(start).as_secs_f64();

        if run_duration > 0.001 {
            total_duration += run_duration;
            measurements.push(run_duration);
            count += 1;
        } else {
            // Very fast function, run it a lot to get a good measurement
            let start = Instant::now();
            for _ in 0..1000 {
                black_box(f());
            }
            let end = Instant::now();
            let run_duration = end.duration_since(start).as_secs_f64();
            total_duration += run_duration;
            measurements.push(run_duration / 1000.0);
            count += 1000;
        }
    }
    let _average = total_duration / f64::from(count);
    let middle = measurements.len() / 2;
    let (left, median, right) =
        measurements.select_nth_unstable_by(middle, |a, b| a.partial_cmp(b).unwrap());
    *median
}

macro_rules! printf {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        std::io::stdout().flush();
    }};
}

macro_rules! printlnf {
    ($($arg:tt)*) => {{
        println!($($arg)*);
        std::io::stdout().flush();
    }};
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
        .filter(|&n| n >= 2 && n <= max_size && (cli.all || is_smooth(n)))
    {
        let input = &mut input[..size];

        // Compute matrix dimensions for some algorithms
        let a = split(size);
        let b = size / a;

        // Filter
        if !match cli.algo {
            Algorithm::Winter => input.len().is_power_of_two(),
            _ => true,
        } {
            continue;
        }

        // Benchmark
        printf!("{size}");
        match cli.algo {
            Algorithm::Ntt | Algorithm::Winter => {}
            _ => printf!(","),
        }
        let duration = match cli.algo {
            Algorithm::Naive => time(|| ntt::naive::ntt(input)),
            Algorithm::Ntt => {
                let strat = ntt::strategy(size);
                printf!(",");
                let result = time(|| strat.ntt(input));
                goldilocks_ntt::ntt::clear_cache();
                result
            }
            Algorithm::Inverse => time(|| ntt::intt(input)),
            Algorithm::Transpose => time(|| {
                // Representative of the work in six-step NTT.
                permute::transpose(input, (a, b));
                permute::transpose(input, (b, a));
                permute::transpose(input, (a, b));
            }),
            #[cfg(feature = "winter-math")]
            Algorithm::Winter => {
                let twiddles = winter_math::fft::get_twiddles(input.len());
                let mut input = input
                    .iter()
                    .map(|n| winter_math::fields::f64::BaseElement::new((*n).into()))
                    .collect::<Vec<_>>();
                printf!(",");
                time(|| winter_math::fft::evaluate_poly(input.as_mut_slice(), &twiddles))
            }
            #[cfg(not(feature = "winter-math"))]
            Algorithm::Winter => {
                panic!("winter-math feature not enabled")
            }
            Algorithm::Merkle => {
                let n = input.len();
                let input: &[u8] = bytemuck::cast_slice(input);
                assert_eq!(input.len(), n * 8);
                time(|| goldilocks_pcs::k12::merkle_tree(&input))
            }
        };
        let throughput = (size as f64) / duration;
        printlnf!("{duration},{throughput}");
    }
}
