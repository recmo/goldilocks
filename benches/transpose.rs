use clap::{Parser, ValueEnum};
use goldilocks_ntt::{
    bench::{rand_vec, time},
    transpose::{transpose_copy_ro, transpose_copy_wo, transpose_square_pub},
};

#[derive(Clone, Debug, ValueEnum)]
enum Algorithm {
    CopyRo,
    CopyWo,
    Square,
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

    const MAX_SIZE: usize = 1_usize << 32;

    eprintln!("Generating random input");
    let mut input = rand_vec(MAX_SIZE);

    println!("size,duration,throughput");
    for size in 5..=16 {
        let width = cli.aspect_width << size;
        let height = cli.aspect_height << size;
        let size = width * height;
        if size > input.len() {
            break;
        }
        let input = &mut input[..size];

        let duration = time(|| match cli.algo {
            Algorithm::CopyRo => transpose_copy_ro(input, width, height),
            Algorithm::CopyWo => transpose_copy_wo(input, width, height),
            Algorithm::Square => transpose_square_pub(input, width, height),
        });
        let throughput = (size as f64) / duration;
        println!("{size},{duration},{throughput}");
    }
}
