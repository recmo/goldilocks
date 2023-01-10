use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    divisors::{divisors, split},
    ntt_old::transpose_square_stretch,
    permute::{transpose, transpose_copy_ro, transpose_copy_wo, transpose_square_pub},
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    CopyRo,
    CopyWo,
    Square,
    Old,
    New,
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

    /// Aspect ratio of the matrix (width, height)
    #[arg(default_value_t = 1)]
    aspect_width: usize,

    /// Aspect ratio of the matrix (width, height)
    #[arg(default_value_t = 1)]
    aspect_height: usize,

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

    eprintln!("Generating random input");
    let mut input = rand_vec(1 << 20);

    println!("size,duration,throughput");
    for &size in divisors() {
        let size = size as usize;
        if size < 1024 {
            continue;
        }
        if size > input.len() {
            break;
        }
        let a = split(size);
        let b = size / a;
        let input = &mut input[..size];

        let duration = time(|| match cli.algo {
            Algorithm::CopyRo => transpose_copy_ro(input, b, a),
            Algorithm::CopyWo => transpose_copy_wo(input, b, a),
            Algorithm::Square => transpose_square_pub(input, b, a),
            Algorithm::Old => {
                transpose_square_stretch(input, a, b / a);
            }
            Algorithm::New => transpose(input, (a, b)),
        });
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
