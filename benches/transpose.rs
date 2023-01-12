use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    divisors::{divisors, is_smooth, split},
    ntt_old::transpose_square_stretch,
    permute::{gw18, transpose, transpose_copy_ro, transpose_copy_wo, transpose_square_pub},
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    CopyRo,
    CopyWo,
    Square,
    Old,
    New,
    Lib,
    Gw18,
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

    let max_exp = cli.max_exponent;
    let max_size = 1 << max_exp;
    eprintln!("Generating random input of size 2^{max_exp} = {max_size}");
    let mut input = rand_vec(max_size);

    println!("size,duration,throughput");
    for size in divisors()
        .iter()
        .map(|&n| n as usize)
        .filter(|&n| is_smooth(n) && n < max_size)
    {
        // Compute matrix dimensions for some algorithms
        let a = split(size);
        let b = size / a;
        let input = &mut input[..size];

        // Check support
        let supported = match cli.algo {
            Algorithm::Old => (a >= 2) && a.is_power_of_two() && (a == b || 2 * a == b),
            Algorithm::Square => a == b,
            _ => true,
        };
        if !supported {
            continue;
        }

        let duration = time(|| match cli.algo {
            Algorithm::CopyRo => transpose_copy_ro(input, b, a),
            Algorithm::CopyWo => transpose_copy_wo(input, b, a),
            Algorithm::Square => transpose_square_pub(input, b, a),
            Algorithm::Old => {
                transpose_square_stretch(input, a, b / a);
            }
            Algorithm::New => transpose(input, (a, b)),
            Algorithm::Lib => {
                let mut w = vec![0; a + b];
                transpose::ip_transpose(input, w.as_mut_slice(), a, b);
            }
            Algorithm::Gw18 => {
                gw18::transpose(input, (a, b));
            }
        });
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
